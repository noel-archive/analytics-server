// 🐻‍❄️🐾 Noelware Analytics: Platform to build upon metrics ingested from any source, from your HTTP server to system-level metrics
// Copyright 2022-2023 Noelware <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::to_redis_err;
use analytics_protobufs::analytics_client::AnalyticsClient;
use analytics_protobufs::{ConnectionAckRequest, ConnectionAckResponse};
use anyhow::{anyhow, Result};
use redis::Value::Nil;
use redis::{FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};
use rsa::{PaddingScheme, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::{Request, Status};

#[derive(Clone, Debug)]
pub struct EndpointKeys {
    pub public: RsaPublicKey,
    pub(crate) private: RsaPrivateKey,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Endpoint {
    pub instance_name: String,
    pub addr: SocketAddr,
    pub api_token: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub keys: Option<EndpointKeys>,
}

impl ToRedisArgs for Endpoint {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
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
        };
    }
}

pub struct EndpointAuth {
    pub(crate) token: String,
}

impl Interceptor for EndpointAuth {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request
            .metadata_mut()
            .insert("Authorization", self.token.parse().unwrap());
        Ok(request)
    }
}

impl Endpoint {
    pub fn new<N: Into<String>, S: Into<SocketAddr>>(instance_name: N, addr: S) -> Self {
        Self {
            instance_name: instance_name.into(),
            addr: addr.into(),
            api_token: None,
            keys: None,
        }
    }

    pub async fn get_grpc_client(
        &self,
    ) -> Result<AnalyticsClient<InterceptedService<Channel, EndpointAuth>>> {
        let mut token: Option<String> = None;
        if let Some(keys) = &self.keys {
            match keys.private.decrypt(
                PaddingScheme::new_pkcs1v15_encrypt(),
                self.api_token.clone().unwrap().as_bytes(),
            ) {
                Ok(dec) => token = Some(String::from_utf8(dec).unwrap()),
                Err(_e) => {}
            }
        }

        if token.is_none() {
            return Err(anyhow!("Unable to determine service token"));
        }

        let channel = Channel::from_shared(format!("grpc://{}", self.addr))
            .unwrap()
            .connect()
            .await?;

        Ok(AnalyticsClient::with_interceptor(
            channel,
            EndpointAuth {
                token: token.unwrap(),
            },
        ))
    }

    pub async fn is_healthy(&self) -> Result<ConnectionAckResponse> {
        let client = self.get_grpc_client().await;
        return match client {
            Ok(mut c) => {
                let ack_res = c.connection_ack(ConnectionAckRequest {}).await;
                match ack_res {
                    Ok(v) => Ok(v.into_inner()),
                    Err(e) => Err(anyhow::Error::from(e)),
                }
            }
            Err(e) => Err(anyhow!(e.to_string())),
        };
    }
}
