use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::config::CONFIG;
use crate::models::response::ApiError;

pub struct AuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ApiError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = CONFIG.get().cloned().unwrap();
        // TODO: Match Authorization header against production API token, for people wanting to register with our production instance.
        return match request.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, ApiError {
                code: Status::Unauthorized.code.to_string(),
                message: "No authorization header specified.".into()
            })),
            Some(v) if config.secret_key.is_some() && config.secret_key.unwrap() == v.to_string() => Outcome::Success(AuthGuard {}),
            Some(_) => Outcome::Failure((Status::Unauthorized, ApiError {
                code: Status::Forbidden.code.to_string(),
                message: "Invalid secret key".into()
            }))
        }
    }
}