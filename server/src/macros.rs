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

#[macro_export]
macro_rules! expand_body {
    ($body:expr, $payload:expr) => {{
        while let Some(chunk) = $payload.next().await {
            let chunk = chunk?;
            if ($body.len() + chunk.len()) > 262_144 {
                return Err::<::actix_web::HttpResponse, Box<dyn std::error::Error + 'static>>(
                    Box::new(::actix_web::error::ErrorPayloadTooLarge("heck!")),
                );
            }

            $body.extend_from_slice(&chunk);
        }

        Ok::<(), Box<dyn std::error::Error + 'static>>(())
    }};
}
