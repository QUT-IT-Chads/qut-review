use serde::{Deserialize, Serialize};
use rocket::serde::uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: Uuid,
    email: String,
    hashed_password: String,
    settings: UserSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSettings {}
