use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::rc::Rc;
use redis::{FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};
use redis::Value::Nil;
use serde::{Serialize, Deserialize};
use serde_json::json;
use rsa::{RsaPublicKey, RsaPrivateKey};
use crate::to_redis_err;

#[derive(Clone, Debug)]
pub struct EndpointKeys {
    pub public: RsaPublicKey,
    pub(crate) private: RsaPrivateKey,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Endpoint {
    pub instance_name: String,
    pub addr: SocketAddr,
    pub api_token: Option<String>
}

impl ToRedisArgs for Endpoint {
    fn write_redis_args<W>(&self, out: &mut W) where W: ?Sized + RedisWrite {
        let endpoint_json = json!(self);
        out.write_arg(endpoint_json.to_string().as_bytes());
    }
}

impl FromRedisValue for Endpoint {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        return if *v == Nil {
            Err(to_redis_err!("Entry is nil"))
        } else {
            Ok(serde_json::from_str(String::from_redis_value(v).unwrap().as_str()).unwrap())
        }
    }
}

impl Endpoint {
    pub fn new<N: Into<String>, S: Into<SocketAddr>>(instance_name: N, addr: S) -> Self {
        Self { instance_name: instance_name.into(), addr: addr.into(), api_token: None }
    }
    pub fn set_api_key(&mut self, token: String) {
        self.api_token = Some(token)
    }
}