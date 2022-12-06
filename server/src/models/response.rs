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

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

/// Represents a response to an REST request.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T>
where
    T: Serialize + Debug,
{
    success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<ApiError>>,
}

/// Represents an object represents a detailed error about the REST response
/// and why it failed.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    code: i32,
    message: String,
}

/// Empty struct to keep the Rust compiler happy!
#[derive(Debug, Serialize, Deserialize)]
pub struct Empty;

/// Returns a new [`ApiResponse`] struct for a successful REST request.
pub fn new_response<T>(data: T) -> ApiResponse<T>
where
    T: Serialize + Debug,
{
    ApiResponse {
        success: true,
        errors: None,
        data: Some(data),
    }
}

/// Returns a new [`ApiResponse`] struct for a failed REST request.
pub fn new_err_resp<R, S>(code: i32, message: S) -> ApiResponse<R>
where
    R: Serialize + Debug,
    S: Into<String>,
{
    ApiResponse {
        success: false,
        data: None,
        errors: Some(vec![ApiError {
            code,
            message: message.into(),
        }]),
    }
}
