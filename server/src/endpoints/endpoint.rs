use std::net::SocketAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Endpoint {
    pub instance_name: String,
    pub sa: SocketAddr
}

impl Endpoint {
    pub fn new<S: Into<SocketAddr>>(instance_name: String, sa: S) -> Self {
        Self { instance_name, sa: sa.into() }
    }
}