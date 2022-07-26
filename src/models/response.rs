use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Responder, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MessageResponse {
    pub message: String,
}
