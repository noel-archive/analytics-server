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

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::endpoints::endpoint::Endpoint;
    use crate::endpoints::endpoint_manager::EndpointManager;
    use crate::sentinel::SentinelManager;
    use crate::setup_utils::setup_logging;
    use dotenv::var;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio_test::block_on;

    #[test]
    fn test_redis() {
        println!("Working directory: {:?}", std::env::current_dir().unwrap());
        Config::load(Some(var("ANALYTICS_SERVER_CONFIG_FILE").unwrap()))
            .expect("Failed to load config");
        let config = Config::get().unwrap();
        setup_logging(config).unwrap();
        block_on(async move {
            let sentinel_manager = Arc::new(Mutex::new(SentinelManager::new(config.clone())));
            sentinel_manager.lock().await.setup().await;
            let endpoint_manager = Arc::new(Mutex::new(EndpointManager::new(sentinel_manager)));
            let mut em = endpoint_manager.lock().await;
            info!("Test endpoint creation...");
            info!(
                "{:?}",
                em.add_endpoint(Endpoint::new(
                    "waff",
                    "127.0.0.1:10240".parse::<SocketAddr>().unwrap()
                ))
                .await
            );
            info!("Test get endpoints...");
            info!("{:?}", em.get_endpoints().await);
            info!("Keys: {:?}", em.get_keys("waff"));
            info!("Test delete endpoint...");
            info!("{:?}", em.delete_endpoint("waff".into()).await);
        });
    }
}
