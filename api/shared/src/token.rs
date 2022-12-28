use chrono::Utc;
use domain::enums::role::Role;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: Uuid,
    role: Role,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = jsonwebtoken::errors::ErrorKind;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, jsonwebtoken::errors::ErrorKind> {
        fn is_valid(token: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(token))?)
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((
                Status::BadRequest,
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            )),
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => Outcome::Failure((
                        Status::BadRequest,
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature,
                    )),
                    _ => Outcome::Failure((
                        Status::BadRequest,
                        jsonwebtoken::errors::ErrorKind::InvalidToken,
                    )),
                },
            },
        }
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
