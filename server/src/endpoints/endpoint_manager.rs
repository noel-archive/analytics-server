use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use anyhow::anyhow;
use rand::thread_rng;
use redis::{Commands, RedisError, RedisResult};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::sync::Mutex;
use crate::endpoints::endpoint::{Endpoint, EndpointKeys};
use crate::sentinel::SentinelManager;

#[derive(Clone, Debug)]
pub struct EndpointManager {
    redis: Arc<Mutex<SentinelManager>>,
    keys: HashMap<String, EndpointKeys>
}

impl EndpointManager {

    pub fn new(redis: Arc<Mutex<SentinelManager>>) -> Self {
        Self {
            redis,
            keys: HashMap::new()
        }
    }

    pub async fn get_endpoints(&mut self) -> anyhow::Result<Vec<Endpoint>> {
        let mut lock =  self.redis.lock().await;
        info!("{:?}", lock);
        return match lock.get_master().await {
            Ok(mut client) =>  {
                let r = client.hgetall::<_, HashMap<String, Endpoint>>("endpoints");
                match r {
                    Ok(v) => Ok(v.values().cloned().collect()),
                    Err(e) => Err(anyhow::Error::from(e))
                }
            }
            Err(e) => Err(anyhow::Error::from(e))
        }
    }

    pub async fn add_endpoint(&mut self, endpoint: Endpoint) -> RedisResult<EndpointKeys> {
        let mut lock = self.redis.lock().await;
        return match lock.get_master().await {
            Ok(mut client) => {
                let r = client.hset::<&str, String, Endpoint, i32>("endpoints", endpoint.clone().instance_name, endpoint.clone().into());
                return match r {
                    Ok(_) => {
                        let private = RsaPrivateKey::new(&mut thread_rng(), 2048);
                        if r.is_err() {
                            return Err(RedisError::from(Error::new(ErrorKind::Other, format!("Failed to create rsa private key: {}", r.unwrap_err().to_string()))));
                        }
                        let private_key = private.unwrap();
                        let public = RsaPublicKey::from(&private_key);
                        let new_keys = EndpointKeys { private: private_key, public };
                        self.keys.insert(endpoint.instance_name, new_keys.clone());
                        return Ok(new_keys)
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    pub async fn delete_endpoint(&mut self, name: String) -> RedisResult<i32> {
        return match self.redis.lock().await.get_master().await {
            Ok(mut client) => {
                self.keys.remove(&name);
                client.hdel::<&str, String, i32>("endpoints", name)
            }
            Err(e) => Err(e)
        }
    }

    pub fn get_keys<S: Into<String>>(&mut self, instance: S) -> anyhow::Result<EndpointKeys> {
        match self.keys.get(&instance.into()) {
            None => Err(anyhow!("No key entry found!")),
            Some(v) => Ok(v.clone())
        }
    }


}