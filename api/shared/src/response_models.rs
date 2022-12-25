use rocket::serde::Serialize;
use rocket::Responder;
use rocket_okapi::JsonSchema;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 404)]
    NotFound(String),

    NotInDemoMode(String),
    NoDemoData(String),
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AuthToken {
    token: String,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResponseMessage {
    pub message: String,
}
