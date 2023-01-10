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

#![feature(backtrace_frames)]

/// Returns the current version of the Analytics server.
pub const VERSION: &str = env!("ANALYTICS_SERVER_VERSION");

/// Returns the current build date of the server.
pub const BUILD_DATE: &str = env!("ANALYTICS_SERVER_BUILD_DATE");

/// Returns the commit hash of the Git repository of Noelware Analytics.
pub const COMMIT_HASH: &str = env!("ANALYTICS_SERVER_COMMIT_HASH");

#[macro_use]
extern crate log;
extern crate core;

pub mod catchers;
pub mod clickhouse;
pub mod config;
pub mod endpoints;
pub mod errors;
pub mod macros;
pub mod middleware;
pub mod models;
pub mod null_writer;
pub mod prisma;
pub mod routes;
pub mod sentinel;
pub mod sentinel_test;
pub mod server;
pub mod setup_utils;
