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

use analytics_protobufs::analytics_client;
use rocket::get;
use rocket::serde::json::Json;
use serde::Serialize;

use crate::models::response::ApiResponse;
use crate::{models::response::new_response, BUILD_DATE, COMMIT_HASH, VERSION};

#[derive(Debug, Serialize)]
pub struct MainResponse {
    message: String,
    docs_url: String,
}

#[derive(Debug, Serialize)]
pub struct InfoResponse {
    version: String,
    commit_hash: String,
    build_date: String,
    vendor: String,
    product: String,
}

#[get("/")]
pub async fn index() -> Json<ApiResponse<MainResponse>> {
    Json(new_response(MainResponse {
        message: "Hello, world!".into(),
        docs_url: "https://analytics.noelware.org/docs/server".into(),
    }))
}

#[get("/heartbeat")]
pub async fn heartbeat() -> &'static str {
    "OK"
}

#[get("/info")]
pub async fn info() -> Json<ApiResponse<InfoResponse>> {
    Json(new_response(InfoResponse {
        version: VERSION.into(),
        commit_hash: COMMIT_HASH.into(),
        build_date: BUILD_DATE.into(),
        vendor: "Noelware".into(),
        product: "analytics-server".into(),
    }))
}
