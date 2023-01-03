use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;

#[derive(Debug, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AuthToken {
    pub token: String,
}

#[derive(Debug, Serialize, JsonSchema, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ResponseMessage {
    pub message: Option<String>,
}
