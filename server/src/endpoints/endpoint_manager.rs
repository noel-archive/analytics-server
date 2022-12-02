use redis::{Commands};
use serde_json::json;
use crate::endpoints::endpoint::Endpoint;

pub struct EndpointManager {
    redis: redis::Client,
    pub endpoints: Vec<Endpoint>,
}

impl EndpointManager {

    pub fn new(redis: redis::Client) -> Self {
        Self {
            redis,
            endpoints: vec![]
        }
    }

    pub fn add_endpoint(&mut self, endpoint: Endpoint) {
        self.endpoints.push(endpoint.clone());
    }


}