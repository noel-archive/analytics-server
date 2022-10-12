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

use std::net::SocketAddr;

use actix_web::{
    self,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;

use crate::{
    config::{Config, ServerConfig},
    routes,
};

#[derive(Debug, Clone)]
pub struct Server {
    config: &'static Config,
}

impl Default for Server {
    fn default() -> Self {
        Server::new()
    }
}

impl Server {
    pub fn new() -> Server {
        let config = Config::get().unwrap();
        Server { config }
    }

    pub async fn launch(self) -> Result<()> {
        info!("launching server...");

        let config = self.config.clone();
        let server_cfg = config.server.unwrap_or(ServerConfig {
            log_requests: Some(true),
            host: Some("0.0.0.0".into()),
            port: Some(9292),
        });

        let addr = match &server_cfg.host {
            Some(host) => {
                let port = server_cfg.port.unwrap_or(9292);
                format!("{}:{}", host, port)
                    .parse::<SocketAddr>()
                    .expect("unable to parse host:port to SocketAddr")
            }
            None => {
                let port = server_cfg.port.unwrap_or(9292);
                format!("0.0.0.0:{}", port)
                    .parse::<SocketAddr>()
                    .expect("unable to parse host:port to SocketAddr")
            }
        };

        info!("launching server on {addr}!");
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(self.clone()))
                .wrap(Logger::new("%r ~> %s [%b bytes; %D ms]").log_target("actix::request"))
                .route("/", web::get().to(routes::main::index))
                .route("/info", web::get().to(routes::main::info))
                .route("/heartbeat", web::get().to(routes::main::heartbeat))
        })
        .bind(addr)?
        .run()
        .await?;

        Ok(())
    }
}
