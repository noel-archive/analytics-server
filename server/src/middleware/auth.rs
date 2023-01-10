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

use crate::config::CONFIG;
use crate::models::response::ApiError;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct AuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ApiError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = CONFIG.get().cloned().unwrap();
        // TODO: Match Authorization header against production API token, for people wanting to register with our production instance.
        return match request.headers().get_one("Authorization") {
            None => Outcome::Failure((
                Status::Unauthorized,
                ApiError {
                    code: Status::Unauthorized.code.to_string(),
                    message: "No authorization header specified.".into(),
                },
            )),
            Some(v)
                if config.secret_key.is_some() && config.secret_key.unwrap() == *v =>
            {
                Outcome::Success(AuthGuard {})
            }
            Some(_) => Outcome::Failure((
                Status::Unauthorized,
                ApiError {
                    code: Status::Forbidden.code.to_string(),
                    message: "Invalid secret key".into(),
                },
            )),
        };
    }
}
