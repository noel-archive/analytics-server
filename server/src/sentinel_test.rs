
#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio_test::block_on;
    use dotenv::var;
    use tokio::sync::Mutex;
    use crate::config::Config;
    use crate::endpoints::endpoint::Endpoint;
    use crate::endpoints::endpoint_manager::EndpointManager;
    use crate::routes::main::info;
    use crate::sentinel::SentinelManager;
    use crate::setup_utils::setup_logging;

    #[test]
    fn test_redis() {
        println!("Working directory: {:?}", std::env::current_dir().unwrap());
        Config::load(Some(var("ANALYTICS_SERVER_CONFIG_FILE").unwrap())).expect("Failed to load config");
        let config = Config::get().unwrap();
        setup_logging(config).unwrap();
        block_on(async move {
            let sentinel_manager = Arc::new(Mutex::new(SentinelManager::new(config.clone())));
            sentinel_manager.lock().await.setup().await;
            let endpoint_manager = Arc::new(Mutex::new(EndpointManager::new(sentinel_manager)));
            let mut em =  endpoint_manager.lock().await;
            info!("Test endpoint creation...");
            info!("{:?}", em.add_endpoint(Endpoint::new("waff", "127.0.0.1:10240".parse::<SocketAddr>().unwrap())).await);
            info!("Test get endpoints...");
            info!("{:?}", em.get_endpoints().await);
            info!("Keys: {:?}", em.get_keys("waff"));
            info!("Test delete endpoint...");
            info!("{:?}", em.delete_endpoint("waff".into()).await);
        });
    }
}