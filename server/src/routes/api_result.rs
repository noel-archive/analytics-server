use std::io::Cursor;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug)]
pub struct ApiError {
    pub status: u16,
    pub message: String,
}

type ApiResult<R> = Result<R, ApiError>;

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let v = json!(self).to_string();
        Response::build()
            .status(Status::from_code(self.status).unwrap_or(Status::new(self.status)))
            .sized_body(v.len(), Cursor::new(v))
            .ok()
    }
}