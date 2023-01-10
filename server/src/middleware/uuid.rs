use rocket::data::Outcome;
use rocket::http::Status;
use rocket::Request;
use rocket::request::FromRequest;
use uuid::Uuid;
use crate::models::response::ApiError;

pub struct UuidParse;