use rocket::catch;
use crate::models::response::{ApiResponse, Empty, new_err_resp};

#[catch(422)]
pub async fn malformed_entity() -> ApiResponse<Empty> {
    new_err_resp::<_, &str>(422, "We were unable to process your request!")
}