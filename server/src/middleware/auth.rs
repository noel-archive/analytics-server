use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::routes::api_result::ApiError;

pub struct AuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ApiError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        todo!()
    }
}