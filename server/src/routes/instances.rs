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

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{Mutex};
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use rsa::PaddingScheme;
use rsa::pkcs8::{LineEnding, EncodePublicKey};
use serde::{Deserialize, Serialize};
use uuid::{Uuid, uuid};
use crate::endpoints::endpoint::Endpoint;
use crate::endpoints::endpoint_manager::EndpointManager;
use crate::middleware::auth::AuthGuard;
use crate::models::response::{ApiError, ApiResponse, Empty, empty_response, new_err_resp, new_err_resp_from_err, new_response};
use crate::routes::main::info;

#[derive(Deserialize)]
pub struct InstanceInitRequest {
    pub addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceInitResponse {
    pub pub_key: String,
    pub uuid: String
}

#[derive(Deserialize)]
pub struct InstanceFinalizeRequest {
    pub api_token: String,
}

#[post("/<id>/init", format = "json", data = "<body>")]
pub async fn instance_init(auth: Result<AuthGuard, ApiError>, id: String, body: Json<InstanceInitRequest>, manager: &State<Arc<Mutex<EndpointManager>>>) -> ApiResponse<InstanceInitResponse> {
    return match Uuid::parse_str(id.as_str()) {
        Ok(id) => {
            if let Err(e) = auth { return new_err_resp_from_err(e) }
            let mut endpoint_manager = manager.lock().await;
            let parsed = body.addr.parse::<SocketAddr>();
            if parsed.is_err() { return new_err_resp::<_, String>(400, "Invalid address specified, must be ip:port".into()) }
            let existing = endpoint_manager.get_endpoint(id.to_string()).await;
            if let Ok(e) = existing {
                endpoint_manager.drop_key(e.instance_name);
            }
            let endpoint = Endpoint::new(id.to_string(), parsed.unwrap());
            return match endpoint_manager.add_endpoint(endpoint.clone()).await {
                Ok(keys) => {
                    let pub_key = keys.public.to_public_key_pem(LineEnding::default());
                    return match pub_key {
                        Ok(key) => new_response(InstanceInitResponse {
                            pub_key: key,
                            uuid: id.to_string(),
                        }),
                        Err(e) => {
                            warn!("Failed to generate public key: {}", e);
                            new_err_resp::<InstanceInitResponse, String>(500, "Failed to encode public key!".into())
                        }
                    }
                }
                Err(e) => new_err_resp::<_, String>(500, e.to_string())
            }
        },
        Err(_) => new_err_resp(400, "Bad Uuid")
    };
}

#[post("/<id>/finalize", format = "json", data = "<body>")]
pub async fn instance_finalize(auth: Result<AuthGuard, ApiError>, id: String, body: Json<InstanceFinalizeRequest>, manager: &State<Arc<Mutex<EndpointManager>>>) -> ApiResponse<Empty> {
    if let Err(e) = auth { return new_err_resp_from_err(e) }
    let mut endpoint_manager = manager.lock().await;
    return match endpoint_manager.get_endpoint(id.clone()).await {
        Err(_) => empty_response(Some(Status::NotFound)),
        Ok(mut e) => {
            let keys = endpoint_manager.get_keys(e.clone().instance_name).unwrap();
            match base64::decode(body.api_token.clone()) {
                Err(_) => new_err_resp(400, "Failed to decode base64 encoded token!"),
                Ok(dec) => {
                    let decrypt_res = keys.private.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &dec[..]);
                    if decrypt_res.is_err() {
                        return new_err_resp(400, "Failed to decrypt API token, check your signing method!");
                    }
                    match endpoint_manager.store_api_key(&mut e, body.api_token.clone()).await {
                        Ok(v) => if !v { new_err_resp::<Empty, &str>(500, "Failed to update redis entry!") } else {
                            tokio::spawn(async move {
                                info!("{:?}", e.is_healthy().await);
                            });
                            empty_response(Some(Status::Accepted))
                        },
                        Err(e) => new_err_resp(500, e.to_string())
                    }
                }
            }
        }
    }
}
