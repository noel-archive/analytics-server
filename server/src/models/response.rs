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
use std::io::Cursor;
use rocket::{Request, Response};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;

use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents a response to an REST request.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T>
    where
        T: Serialize + Debug,
{
    #[serde(skip_serializing)]
    status: Option<u16>,

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
    pub code: String,
    pub message: String,
}

/// Empty struct to keep the Rust compiler happy!
#[derive(Debug, Serialize, Deserialize)]
pub struct Empty;

pub fn empty_response(status: Option<Status>) -> ApiResponse<Empty> {
    ApiResponse {
        status: status.map(|x| x.code),
        data: None,
        success: false,
        errors: None,
    }
}

impl<T> ApiResponse<T> where T: Serialize + Debug {
    fn is_empty(&self) -> bool {
        !self.success && self.data.is_none() && self.errors.is_none()
    }
}

/// Returns a new [`ApiResponse`] struct for a successful REST request.
pub fn new_response<T>(data: T) -> ApiResponse<T>
    where
        T: Serialize + Debug,
{
    ApiResponse {
        status: None,
        success: true,
        errors: None,
        data: Some(data),
    }
}

pub fn new_response_with_status<T>(status: u16, data: T) -> ApiResponse<T>
    where
        T: Serialize + Debug,
{
    ApiResponse {
        status: Some(status),
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
        status: None,
        success: false,
        data: None,
        errors: Some(vec![ApiError {
            code: code.to_string(),
            message: message.into(),
        }]),
    }
}

pub fn new_err_resp_from_err<R>(err: ApiError) -> ApiResponse<R> where R: Serialize + Debug {
    ApiResponse {
        status: None,
        success: false,
        data: None,
        errors: Some(vec![err]),
    }
}

impl<'r, S> Responder<'r, 'static> for ApiResponse<S> where S: Serialize + Debug {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        if self.is_empty() {
            return Response::build()
                .status(self.status.map(|x| Status::from_code(x).unwrap_or(Status::NoContent)).unwrap_or(Status::NoContent))
                .ok();
        }
        let serialised = json!(self);
        let str = serialised.to_string();
        if let Some(errs) = self.errors {
            return Response::build()
                .sized_body(str.len(), Cursor::new(str))
                .status(Status::from_code(errs[0].code.parse::<u16>().unwrap()).unwrap_or(Status::InternalServerError))
                .header(ContentType::JSON)
                .ok();
        }
        Response::build()
            .sized_body(str.len(), Cursor::new(str))
            .status(self.status.map(|x| Status::from_code(x).unwrap_or(Status::Ok)).unwrap_or(Status::Ok))
            .header(ContentType::JSON)
            .ok()
    }
}