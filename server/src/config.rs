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

use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::env::var;
use std::fmt::Write as _;
use std::fs::File;
use std::path::Path;

pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The DSN to connect to Sentry for error handling.
    pub sentry_dsn: Option<String>,

    /// Configuration for ClickHouse, which is used to enable the Events API.
    pub clickhouse: Option<ClickHouseConfig>,

    /// If the web UI should be enabled when running the server. If this is disabled,
    /// all rest APIs will land on `/` instead of `/api`.
    pub frontend: Option<bool>,

    /// Configuration for the logging system.
    pub logging: Option<LogConfig>,

    /// Configuration for the server itself.
    pub server: Option<ServerConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    pub logstash_url: Option<String>,
    pub level: Option<String>,
    pub json: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClickHouseConfig {
    pub min_connections_in_pool: Option<u16>, // defaults to 10
    pub max_connections_in_pool: Option<u16>, // defaults to 20
    pub use_lz4_compression: Option<bool>,    // defaults to "false"
    pub database: Option<String>,             // defaults to "analytics"
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    /// If the server should log requests or not.
    pub log_requests: Option<bool>,

    /// The port the server should bind to. Default is `9292`.
    pub port: Option<i16>,

    /// The host the server should bind to. Default is `0.0.0.0` or `::1`.
    pub host: Option<String>,
}

impl ToString for ClickHouseConfig {
    #[allow(unused_assignments)]
    fn to_string(&self) -> String {
        // is this a bad idea? probably.
        // if you think this is bad, smash star on this repository :)
        let mut url = String::from("tcp://");

        if self.username.is_some() && self.password.is_some() {
            let pass = self.password.as_ref().unwrap();
            let user = self.username.as_ref().unwrap();

            url.push_str(user.as_str());
            url.push(':');
            url.push_str(pass.as_str());
            url.push('@');
        }

        url.push_str(
            self.host
                .as_ref()
                .unwrap_or(&"localhost".to_string())
                .as_str(),
        );

        url.push(':');
        url.push_str(self.port.as_ref().unwrap_or(&9000).to_string().as_str());

        // now we're at the point of:
        // tcp://<username>:<password>@<host>:<port>
        //
        // now we need to append db name and parameters. this is fine.
        url.push('/');
        url.push_str(
            self.database
                .as_ref()
                .unwrap_or(&"telemetry".to_string())
                .as_str(),
        );

        let mut prefix = '?';
        match self.use_lz4_compression {
            Some(b) if b => {
                url.push_str("?compression=lz4");
                if prefix == '?' {
                    prefix = '&';
                }
            }

            _ => {}
        }

        if let Some(max_conn) = self.max_connections_in_pool {
            url.push(prefix);
            let _ = write!(url, "pool_max={}", max_conn);

            if prefix == '?' {
                prefix = '&';
            }
        }

        if let Some(min_conn) = self.min_connections_in_pool {
            url.push(prefix);
            let _ = write!(url, "pool_min={}", min_conn);

            if prefix == '?' {
                prefix = '&';
            }
        }

        url
    }
}

impl Config {
    /// Pulls the configuration from the environment variables as a source.
    ///
    /// | Name                                 | Environment Variable Key                    | Required? | Type     |
    /// | :----------------------------------- | :------------------------------------------ | :-------- | :------- |
    /// | `clickhouse.min_connections_in_pool` | ANALYTICS_SERVER_CLICKHOUSE_MIN_CONNECTIONS | false     | u16      |
    /// | `clickhouse.max_connections_in_pool` | ANALYTICS_SERVER_CLICKHOUSE_MAX_CONNECTIONS | false     | u16      |
    /// | `clickhouse.use_lz4_compression`     | ANALYTICS_SERVER_CLICKHOUSE_LZ4_COMPRESSION | false     | bool     |
    /// | `clickhouse.database`                | ANALYTICS_SERVER_CLICKHOUSE_DATABASE        | false     | String   |
    /// | `clickhouse.username`                | ANALYTICS_SERVER_CLICKHOUSE_USERNAME        | false     | String   |
    /// | `clickhouse.password`                | ANALYTICS_SERVER_CLICKHOUSE_PASSWORD        | false     | String   |
    /// | `clickhouse.host`                    | ANALYTICS_SERVER_CLICKHOUSE_HOST            | false     | String   |
    /// | `clickhouse.port`                    | ANALYTICS_SERVER_CLICKHOUSE_PORT            | false     | String   |
    /// | `logging.logstash_url`               | ANALYTICS_SERVER_LOGSTASH_URL               | false     | URL      |
    /// | `logging.level`                      | ANALYTICS_SERVER_LOG_LEVEL                  | false     | LogLevel |
    /// | `logging.json`                       | ANALYTICS_SERVER_LOG_JSON                   | false     | bool     |
    /// | `server.log_requests`                | ANALYTICS_SERVER_HTTP_LOG_REQUESTS          | false     | bool     |
    /// | `server.port`                        | ANALYTICS_SERVER_HTTP_PORT (or `PORT`)      | false     | u16      |
    /// | `server.host`                        | ANALYTICS_SERVER_HTTP_HOST (or `HOST`)      | false     | String   |
    /// | `sentry_dsn`                         | ANALYTICS_SERVER_SENTRY_DSN                 | false     | String   |
    /// | `frontend`                           | ANALYTICS_SERVER_FRONTEND                   | false     | bool     |
    fn from_env() -> Config {
        Config {
            sentry_dsn: var("ANALYTICS_SERVER_SENTRY_DSN").ok(),
            frontend: var("ANALYTICS_SERVER_FRONTEND").ok().map(|p| {
                p.parse::<bool>()
                    .expect("Unable to convert environment variable value to bool.")
            }),

            clickhouse: Some(ClickHouseConfig {
                max_connections_in_pool: var("ANALYTICS_SERVER_CLICKHOUSE_MAX_CONNECTIONS")
                    .ok()
                    .map(|p| {
                        p.parse::<u16>()
                            .expect("Unable to convert environment variable value to u16.")
                    }),

                min_connections_in_pool: var("ANALYTICS_SERVER_CLICKHOUSE_MIN_CONNECTIONS")
                    .ok()
                    .map(|p| {
                        p.parse::<u16>()
                            .expect("Unable to convert environment variable value to u16.")
                    }),

                use_lz4_compression: var("ANALYTICS_SERVER_CLICKHOUSE_LZ4_COMPRESSION").ok().map(
                    |p| {
                        p.parse::<bool>()
                            .expect("Unable to convert environment variable value to bool.")
                    },
                ),

                username: var("ANALYTICS_SERVER_CLICKHOUSE_USERNAME").ok(),
                password: var("ANALYTICS_SERVER_CLICKHOUSE_PASSWORD").ok(),
                database: var("ANALYTICS_SERVER_CLICKHOUSE_DATABASE").ok(),
                host: var("ANALYTICS_SERVER_CLICKHOUSE_HOST").ok(),
                port: var("ANALYTICS_SERVER_CLICKHOUSE_PORT").ok().map(|p| {
                    p.parse::<u16>()
                        .expect("Unable to convert environment variable value to u16.")
                }),
            }),

            logging: Some(LogConfig {
                logstash_url: var("ANALYTICS_SERVER_LOGSTASH_URL").ok(),
                level: var("ANALYTICS_SERVER_LOG_LEVEL").ok(),
                json: var("ANALYTICS_SERVER_LOG_JSON").ok().map(|p| {
                    p.parse::<bool>()
                        .expect("Unable to convert environment variable value to bool.")
                }),
            }),

            server: Some(ServerConfig {
                log_requests: var("ANALYTICS_SERVER_HTTP_LOG_REQUESTS").ok().map(|p| {
                    p.parse::<bool>()
                        .expect("Unable to convert environment variable value to bool.")
                }),

                host: var("ANALYTICS_SERVER_HTTP_HOST").ok(),
                port: var("ANALYTICS_SERVER_HTTP_PORT").ok().map(|p| {
                    p.parse::<i16>()
                        .expect("Unable to convert environment variable value to i16.")
                }),
            }),
        }
    }

    fn from_file<P>(path: P) -> Result<Config>
    where
        P: Into<String> + AsRef<Path>,
    {
        let fd = File::open(path)?;
        Ok(serde_yaml::from_reader(fd)?)
    }

    /// Loads the configuration file from an optional file path. If the path is `None`, then
    /// it will attempt to load in the root directory with the `config.yml` file, if any. If not,
    /// it'll use the system environment variables as a fallback, read more in the documentation:
    /// https://analytics.noelware.org/docs/server/self-hosting#configuration
    pub fn load<P>(path: Option<P>) -> Result<()>
    where
        P: Into<String> + AsRef<Path>,
    {
        // don't attempt to call this again if it was already loaded.
        if CONFIG.get().is_some() {
            warn!("load() was called more than once!");
            return Ok(());
        }

        match path {
            Some(p) => match Config::from_file(p) {
                Ok(config) => {
                    CONFIG.set(config).unwrap();
                    Ok(())
                }

                Err(e) => {
                    // we use the println macro because Config::load() is usually loaded in in main function
                    // before fern is initialized.
                    println!(
                        "[preinit warn] Unable to load configuration in path due to [{}], using system environment variables as a fallback",
                        e
                    );

                    CONFIG.set(Config::from_env()).unwrap();
                    Ok(())
                }
            },

            // If no path was specified, call the `load` function again with
            // a default path that most people will use.
            None => Config::load(Some("./config.yml")),
        }
    }

    /// Since the configuration is initialized only once, this will mostly return as `Some(<config>)`, so
    /// most of the times, using `.unwrap()` is ok, unless [#load][Config.load] was not called at all.
    pub fn get() -> Option<&'static Config> {
        CONFIG.get()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn url_tests_without_params_and_auth() {
        let config = crate::config::ClickHouseConfig {
            min_connections_in_pool: None,
            max_connections_in_pool: None,
            use_lz4_compression: None,
            database: Some("telemetry".into()),
            username: None,
            password: None,
            host: Some("localhost".into()),
            port: Some(9000),
        };

        let url = config.to_string();
        assert_eq!(url, "tcp://localhost:9000/telemetry");
    }

    #[test]
    fn url_tests_with_auth_but_not_params() {
        let config = crate::config::ClickHouseConfig {
            min_connections_in_pool: None,
            max_connections_in_pool: None,
            use_lz4_compression: None,
            database: Some("telemetry".into()),
            username: Some("noel".into()),
            password: Some("noelisthebest".into()),
            host: Some("localhost".into()),
            port: Some(9000),
        };

        let url = config.to_string();
        assert_eq!(url, "tcp://noel:noelisthebest@localhost:9000/telemetry");
    }

    #[test]
    fn url_tests_with_params_and_auth() {
        let config = crate::config::ClickHouseConfig {
            min_connections_in_pool: Some(10),
            max_connections_in_pool: Some(300),
            use_lz4_compression: Some(true),
            database: Some("telemetry".into()),
            username: Some("noel".into()),
            password: Some("noelisthebest".into()),
            host: Some("localhost".into()),
            port: Some(9000),
        };

        let url = config.to_string();
        assert_eq!(url, "tcp://noel:noelisthebest@localhost:9000/telemetry?compression=lz4&pool_max=300&pool_min=10");

        let config_2 = crate::config::ClickHouseConfig {
            min_connections_in_pool: Some(69),
            max_connections_in_pool: Some(420),
            use_lz4_compression: None,
            database: Some("telemetry".into()),
            username: Some("noel".into()),
            password: Some("noelisthebest".into()),
            host: Some("localhost".into()),
            port: Some(9000),
        };

        let url2 = config_2.to_string();
        assert_eq!(
            url2,
            "tcp://noel:noelisthebest@localhost:9000/telemetry?pool_max=420&pool_min=69"
        );
    }
}
