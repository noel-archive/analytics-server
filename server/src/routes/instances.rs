use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{Mutex};
use rocket::{post, State};
use rocket::serde::json::Json;
use rsa::pkcs8::{LineEnding, EncodePublicKey};
use serde::{Deserialize, Serialize};
use crate::endpoints::endpoint::Endpoint;
use crate::endpoints::endpoint_manager::EndpointManager;
use crate::models::response::{ApiResponse, new_err_resp, new_response};

#[derive(Deserialize)]
pub struct InstanceInitRequest {
    pub addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceInitResponse {
    pub pub_key: String,
}

#[post("/<id>/init", format = "json", data = "<body>")]
pub async fn instance_init(id: String, body: Json<InstanceInitRequest>, manager: &State<Arc<Mutex<EndpointManager>>>) -> Json<ApiResponse<InstanceInitResponse>> {
    let mut endpoint_manager = manager.lock().await;
    let parsed = body.addr.parse::<SocketAddr>();
    if parsed.is_err() { return Json(new_err_resp::<InstanceInitResponse, String>(400, "Invalid address specified, must be ip:port".into())) }
    return match endpoint_manager.add_endpoint(Endpoint::new(id.clone(), parsed.unwrap())).await {
        Ok(keys) => {
            let pub_key = keys.public.to_public_key_pem(LineEnding::default());
            return match pub_key {
                Ok(key) => Json(new_response(InstanceInitResponse {
                    pub_key: key
                })),
                Err(e) => {
                    warn!("Failed to generate public key: {}", e);
                    Json(new_err_resp::<InstanceInitResponse, String>(500, "Failed to encode public key!".into()))
                }
            }
        }
        Err(e) => Json(new_err_resp::<InstanceInitResponse, String>(500, e.to_string()))
    }
}