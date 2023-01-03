use chrono::Utc;
use domain::enums::role::Role;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Responder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use schemars::JsonSchema;
use std::env;
use uuid::Uuid;

use crate::response_models::ResponseMessage;

#[derive(Responder, Serialize, Deserialize, Debug, JsonSchema)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
    #[response(status = 498)]
    InvalidToken(Option<String>),
    #[response(status = 498)]
    ExpiredSignature(Option<String>),
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Claims {
    pub sub: Uuid,
    pub role: Role,
    pub exp: usize,
}

#[derive(Debug, JsonSchema)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = (Status, Json<ResponseMessage>);

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, (Status, Json<ResponseMessage>)> {
        fn is_valid(token: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(token))?)
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((
                Status::Unauthorized,
                (
                    Status::Unauthorized,
                    Json(ResponseMessage { message: Some(String::from("No token provided.")) }),
                ),
            )),
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => Outcome::Failure((
                        Status::Unauthorized,
                        (
                            Status::Unauthorized,
                            Json(ResponseMessage {
                                message: Some(String::from("Expired Signature")),
                            }),
                        ),
                    )),
                    _ => Outcome::Failure((
                        Status::Unauthorized,
                        (
                            Status::Unauthorized,
                            Json(ResponseMessage { message: None }),
                        ),
                    )),
                },
            },
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for JWT {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an Bearer token to access. A token can be retrieved by logging in."
                    .to_owned(),
            ),
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}

pub fn create_jwt(id: Uuid, role: Role) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: id,
        role,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
