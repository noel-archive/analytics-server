
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio_test::block_on;
    use dotenv::var;
    use tokio::time::sleep;
    use crate::config::Config;
    use crate::sentinel::SentinelManager;
    use crate::setup_utils::setup_logging;

    #[test]
    fn test_redis() {
        println!("Working directory: {:?}", std::env::current_dir().unwrap());
        Config::load(Some(var("ANALYTICS_SERVER_CONFIG_FILE").unwrap())).expect("Failed to load config");
        let config = Config::get().unwrap();
        setup_logging(config).unwrap();
        let mut sentinel_manager = SentinelManager::new(config.clone());
        block_on(async move {
            sentinel_manager.setup().await;
            let cur_client = sentinel_manager.get_master().await.unwrap();
            info!("Current master address: {:?}", cur_client.get_connection_info().addr.to_string());
            sleep(Duration::from_secs(3600)).await;
            let new_client = sentinel_manager.get_master().await.unwrap();
            info!("Failover master address: {:?}", new_client.get_connection_info().addr.to_string());
        });
    }
}