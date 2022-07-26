use crate::models::response::MessageResponse;
use rocket::serde::json::Json;
pub mod customer;

#[get("/")]
pub fn index() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "Welcome".to_string(),
    })
}
