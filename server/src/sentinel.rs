use std::time::Duration;
use redis::{Client, FromRedisValue, RedisError, RedisResult};
use tokio::time::sleep;
use crate::config::{Config, RedisConfig};
use std::io::{Error, ErrorKind};
use std::string::String;
use async_recursion::async_recursion;


#[derive(Clone)]
pub struct SentinelManager {
    sentinels: Vec<Client>,
    master_name: Option<String>,
    master: Option<Client>,
    config: Config
}

#[async_recursion]
async fn try_unreachable_sentinel(addr: String) {
    let client = Client::open(addr.clone()).unwrap();
    if !client.get_connection_with_timeout(Duration::from_millis(250)).is_ok() {
        debug!("Sentinel {} is still unreachable!", addr);
        sleep(Duration::from_secs(5)).await;
        try_unreachable_sentinel(addr).await;
        return
    }
    debug!("Sentinel {} has become reachable!", addr);
}

impl SentinelManager {

    pub fn new(config: Config) -> Self {
        let redis_config = config.clone().redis;
        Self {
            config,
            sentinels: vec![],
            master_name: redis_config.master_name,
            master: None
        }
    }

    pub async fn get_master(&mut self) -> RedisResult<Client> {
        if let Some(client) = &self.master {
            let redis_conf = self.config.redis.clone();
            return match client.get_connection_with_timeout(Duration::from_secs(2)) {
                Err(_) => {
                    info!("Currently we have a bad connection, reconnecting...");
                    let client = self.find_healthy_sentinel();
                    if client.is_err() { return Err(client.unwrap_err()) }
                    let master_addr = self.get_master_addr(client.unwrap()).await;
                    if master_addr.is_none() { return Err(RedisError::from(Error::new(ErrorKind::Other, "No new master found!"))) }
                    let client = Client::open(self.format_url(redis_conf.clone(), master_addr.unwrap(),false)).unwrap();
                    self.master.replace(client.clone());
                    Ok(client.clone())
                }
                _ => Ok(client.clone())
            }
        }
        Err(RedisError::from(Error::new(ErrorKind::Other, "No master found!")))
    }

    fn find_healthy_sentinel(&self) -> RedisResult<Client> {
        match self.sentinels.clone().iter().find(|&x| x.get_connection().is_ok()) {
            None => Err(RedisError::from(Error::new(ErrorKind::Other, "no healthy sentinels found"))),
            Some(r) => Ok(r.clone())
        }
    }

    fn format_url(&mut self, config: RedisConfig, addr: String, sentinel: bool) -> String {
        format!(
            "{}://{}@{}:{}",
            match config.tls.unwrap_or(false) { true => "rediss", false => "redis" },
            match config.password { Some(password) => format!(":{}", password), None => "".to_string() }, addr,
            match sentinel { true => 26379, false => 6379 }
        )
    }

    pub(crate) async fn setup(&mut self) {
        let config = self.config.clone();
        let redis_conf = config.redis.clone();
        let endpoints = redis_conf.clone().endpoints;
        for addr in endpoints {
            let sentinel_url = self.format_url(redis_conf.clone(), addr.clone(), true);
            let client = Client::open(sentinel_url.clone()).unwrap();
            let mut i = 0;
            while !client.get_connection_with_timeout(Duration::from_millis(250)).is_ok() {
                if i >= 5 {
                    error!("Giving up on connecting to sentinel at {}!", addr);
                    sleep(Duration::from_secs(1)).await;
                    tokio::spawn(try_unreachable_sentinel(sentinel_url.clone()));
                    break;
                }
                info!("Sentinel {} has not become ready", addr);
                sleep(Duration::from_millis(250)).await;
                i += 1;
            }
            if i == 5 { continue }
            info!("Sentinel {} has become ready!", addr);
            let mut conn = client.get_async_connection().await.unwrap();
            let redis_info = redis::cmd("INFO").query_async::<_, String>(&mut conn).await.unwrap();
            let redis_info_split = redis_info.split("\n").collect::<Vec<&str>>();
            let redis_mode = redis_info_split.iter().find(|x| x.contains("redis_mode")).unwrap().split(":").collect::<Vec<&str>>()[1].trim();
            if redis_mode != "sentinel" {
                info!("{} is not a sentinel, running in mode {}!", addr, redis_mode);
                self.master = Some(client);
                return;
            }
            if self.master.is_none() {
                info!("Asking for the master address!");
                match self.get_master_addr(client.clone()).await {
                    Some(master) => {
                        info!("Master address is {}", master.clone());
                        self.master = Some(Client::open(self.format_url(redis_conf.clone(), master.clone(), false)).unwrap());
                        let mut conn = self.master.clone().unwrap().get_connection().unwrap();
                        let role: RedisResult<Vec<redis::Value>> = redis::cmd("ROLE").query(&mut conn);
                        let actual = String::from_redis_value(&role.unwrap()[0]).unwrap();
                        if actual == "master" {
                            info!("Confirmed address {} as the current redis master", master);
                        }
                    },
                    None => warn!("Sentinel did not give us a master address, trying the next sentinel...")
                }
            } else {
                info!("We already have a master, skipping!");
            }
            self.sentinels.push(client);
        }
    }

    pub(crate) async fn get_master_addr(&self, client: Client) -> Option<String> {
        let mut master_addr: Option<String> = None;
        let mut conn = client.get_async_connection().await.unwrap();
        let master_name = self.master_name.clone().unwrap_or("".into());
        let master_addr_result: RedisResult<Vec<String>> = redis::cmd("SENTINEL").arg("get-master-addr-by-name").arg(master_name).query_async(&mut conn).await;
        match master_addr_result {
            Ok(addr) => { master_addr = Some(addr[0].clone()); },
            Err(e) => { error!("Error while getting master addr: {}", e); }
        }
        master_addr
    }

}