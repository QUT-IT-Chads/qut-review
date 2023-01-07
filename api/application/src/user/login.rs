use crate::token::create_jwt;
use domain::models::user::LoginRequest;
use infrastructure::{user::read::db_login_request, ServerState};
use rocket::{http::Status, State};

pub fn login_user(
    user: LoginRequest,
    state: &State<ServerState>,
) -> Result<String, (Status, Option<String>)> {
    let user = db_login_request(&user.email, &user.hashed_password, state)?;

    match create_jwt(user.id, user.role) {
        Ok(token) => Ok(token),
        Err(err) => panic!("Error: Failed to create token for user - {}", err),
    }
}
