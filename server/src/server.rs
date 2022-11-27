// 🐻‍❄️🐾 Noelware Analytics: Platform to build upon metrics ingested from any source, from your HTTP server to system-level metrics
// Copyright 2022 Noelware <team@noelware.org>
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

use std::{sync::Arc};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use anyhow::Result;
use rocket::{Error, Ignite, Rocket, routes};
use analytics_protobufs::analytics_client;

use crate::{
    clickhouse::client::ClickHouse,
    config::Config,
    prisma::{new_client, PrismaClient},
    setup_utils,
    routes::main,
};

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
        let grpc_client = analytics_client::AnalyticsClient::connect(format!("grpc://{}:{}", server_cfg.host, server_cfg.port)).await.unwrap();

        let port = server_cfg.port.unwrap_or(9292);
        let addr = match &server_cfg.host {
            Some(host) => IpAddr::from_str(host.as_str()).expect("Invalid host address specified!"),
            None => IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)),
        };
        let port: u16 = server_cfg.port.and_then(|x| Some(x as u16)).unwrap_or(9292);

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
            .mount("/", routes![main::index, main::heartbeat, main::info])
            .launch()
            .await
    }
}
