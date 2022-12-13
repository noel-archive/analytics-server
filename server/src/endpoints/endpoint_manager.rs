use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::Arc;
use anyhow::anyhow;
use rand::{thread_rng};
use redis::{Commands, RedisResult};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::sync::Mutex;
use crate::endpoints::endpoint::{Endpoint, EndpointKeys};
use crate::sentinel::SentinelManager;
use crate::to_redis_err;

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

    pub async fn get_endpoint(&mut self, name: String) -> anyhow::Result<Endpoint> {
        return match self.redis.lock().await.get_master().await {
            Ok(mut client) => {
                return match client.hget::<&str, String, Endpoint>("endpoints".into(), name) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(anyhow::Error::from(e))
                }
            }
            Err(e) => Err(anyhow::Error::from(e))
        }
    }

    pub async fn get_endpoints(&mut self) -> anyhow::Result<Vec<Endpoint>> {
        return match self.redis.lock().await.get_master().await {
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
        return match self.redis.lock().await.get_master().await {
            Ok(mut client) => {
                let r = client.hset::<&str, String, Endpoint, i32>("endpoints", endpoint.clone().instance_name, endpoint.clone().into());
                return match r {
                    Ok(_) => {
                        let mut rng = thread_rng();
                        let private = RsaPrivateKey::new(&mut rng, 2048);
                        if r.is_err() {
                            return Err(to_redis_err!(format!("Failed to create rsa private key: {}", r.unwrap_err().to_string())));
                        }
                        let private_key = private.unwrap();
                        let public = RsaPublicKey::from(&private_key);
                        let new_keys = EndpointKeys { private: private_key, public };
                        self.keys.insert(endpoint.instance_name.clone(), new_keys.clone());
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

    pub fn drop_key<S: Into<String>>(&mut self, instance: S) -> Option<EndpointKeys> {
        self.keys.remove(&instance.into())
    }

    pub async fn store_api_key(&mut self, e: Endpoint, key: String) -> anyhow::Result<bool>  {
        return match self.redis.lock().await.get_master().await {
            Ok(mut client) =>  {
                let r = client.hget::<&str, String, Endpoint>("endpoints", e.instance_name);
                match r {
                    Ok(mut v) => {
                        v.api_token = Some(key);
                        return Ok(match client.hset::<&str, String, Endpoint, i32>("endpoints", v.clone().instance_name, v) {
                            Ok(i) => i >= 0,
                            Err(e) => {
                                warn!("Failed to update endpoint: {}", e);
                                false
                            }
                        })
                    },
                    Err(e) => Err(anyhow::Error::from(e))
                }
            }
            Err(e) => Err(anyhow::Error::from(e))
        }
    }

}