use crate::token::JWT;
use infrastructure::{user::delete::db_delete_user, ServerState};
use rocket::{http::Status, State};
use uuid::Uuid;

use crate::auth::has_user_permissions;

pub fn delete_user(
    user_id: Uuid,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Option<String>, (Status, Option<String>)> {
    has_user_permissions(&token, &user_id)?;

    db_delete_user(&user_id, state)
}
