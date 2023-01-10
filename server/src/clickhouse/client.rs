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

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use anyhow::Result;
use clickhouse_rs::Pool;
use futures::lock::Mutex;

use crate::config::ClickHouseConfig;

/// Represents some abstraction over the ClickHouse connection pool by keeping
/// track of the calls used in this struct, not by the pool itself.
#[derive(Debug, Clone)]
pub struct ClickHouse {
    /// How many database calls have been used during the server's lifetime.
    calls: Arc<Mutex<AtomicUsize>>,

    /// The connection pool itself.
    pool: Pool,
}

impl ClickHouse {
    pub fn new(config: ClickHouseConfig) -> Result<ClickHouse> {
        let url = config.to_string();
        let pool = Pool::new(url);

        Ok(ClickHouse {
            calls: Arc::new(Mutex::new(AtomicUsize::new(0))),
            pool,
        })
    }

    pub fn calls(&self) -> usize {
        self.calls.try_lock().unwrap().load(Ordering::SeqCst)
    }

    pub async fn ping(&self) -> Result<()> {
        debug!("retrieving a connection...");

        let pool = self.pool.clone();
        let mut handle = pool.get_handle().await?;

        self.calls
            .try_lock()
            .unwrap()
            .fetch_add(1, Ordering::SeqCst);

        handle.ping().await?;
        Ok(())
    }
}
