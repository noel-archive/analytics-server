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

// 1664607600000

#[macro_use]
extern crate log;

use analytics_server::{config::Config, server::Server, setup_utils, COMMIT_HASH, VERSION};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // load dotenv just in case people need it
    dotenv::dotenv()?;

    // load config
    match std::env::var("ANALYTICS_SERVER_CONFIG_FILE") {
        Ok(path) => {
            Config::load(Some(path))?;
        }

        Err(_) => {
            Config::load::<String>(None)?;
        }
    }

    // setup logging and sentry
    let config = Config::get().unwrap();
    setup_utils::setup_logging(config)?;
    setup_utils::setup_sentry(config)?;

    info!(
        "~*~ running Noelware Analytics {} ({}) ~*~",
        VERSION, COMMIT_HASH
    );

    // start server
    let server = Server::new().await?;
    server.launch().await?;

    Ok(())
}
