use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;

#[derive(Debug, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AuthToken {
    pub token: String,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
// TODO: convert `message` to Option<&str>
pub struct ResponseMessage {
    pub message: String,
}
