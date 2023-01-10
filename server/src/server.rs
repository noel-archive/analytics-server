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

use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use rocket::{catchers, routes, Error, Ignite, Rocket};
use tokio::sync::Mutex;

use crate::{
    catchers::*,
    clickhouse::client::ClickHouse,
    config::Config,
    prisma::{new_client, PrismaClient},
    routes::*,
    setup_utils,
};

use crate::endpoints::endpoint_manager::EndpointManager;
use crate::sentinel::SentinelManager;

#[derive(Debug, Clone)]
pub struct Server {
    clickhouse: Arc<ClickHouse>,
    prisma: Arc<PrismaClient>,
    config: &'static Config,
}

impl Server {
    pub async fn new() -> Result<Server> {
        let config = Config::get().unwrap();
        let clickhouse = ClickHouse::new(config.clickhouse.clone().unwrap_or_default())?;

        info!("connecting to postgres!");
        let prisma = new_client().await?;

        info!("connected to postgres, now connecting to clickhouse!");
        clickhouse.ping().await?;

        Ok(Server {
            clickhouse: Arc::new(clickhouse.clone()),
            prisma: Arc::new(prisma),
            config,
        })
    }

    pub async fn launch(self) -> std::result::Result<Rocket<Ignite>, Error> {
        info!("testing clickhouse availibility...");
        let clickhouse = self.clickhouse.clone();
        clickhouse.ping().await.expect("Clickhouse is not ready!");

        info!("clickhouse seems stable! now launching server...");
        let config = self.config.clone();
        let server_cfg = config.server.unwrap_or_default();
        let addr = match &server_cfg.host {
            Some(host) => IpAddr::from_str(host.as_str()).expect("Invalid host address specified!"),
            None => IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)),
        };
        let port: u16 = server_cfg.port.map(|x| x as u16).unwrap_or(9292);

        let sentinel_manager = Arc::new(Mutex::new(SentinelManager::new(self.config.clone())));
        sentinel_manager.lock().await.setup().await;
        let endpoint_manager = Arc::new(Mutex::new(EndpointManager::new(sentinel_manager.clone())));

        // setup panic handler
        info!("installing panic hook");
        setup_utils::setup_panic_hook();

        info!("launching server on {addr}!");
        rocket::build()
            .configure(rocket::Config {
                address: addr,
                port,
                ..rocket::config::Config::default()
            })
            .manage(self.clickhouse.clone())
            .manage(self.prisma.clone())
            .manage(self.config.clone())
            .manage(sentinel_manager)
            .manage(endpoint_manager)
            .mount("/", routes![main::index, main::heartbeat, main::info])
            .mount(
                "/instances",
                routes![instances::instance_init, instances::instance_finalize],
            )
            .register("/", catchers![malformed_entity])
            .launch()
            .await
    }
}
