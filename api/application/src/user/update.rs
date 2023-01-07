use crate::token::JWT;
use domain::enums::role::Role;
use domain::models::user::GetUser;
use domain::models::user::UpdateUser;
use infrastructure::user::update::db_update_user;
use infrastructure::ServerState;
use rocket::http::Status;
use rocket::State;
use uuid::Uuid;

use crate::auth::has_user_permissions;

pub fn update_user(
    user_id: Uuid,
    user: UpdateUser,
    state: &State<ServerState>,
    token: JWT,
) -> Result<GetUser, (Status, Option<String>)> {
    has_user_permissions(&token, &user_id)?;

    if user.role == Role::Admin && token.claims.role != Role::Admin {
        return Err((
            Status::Unauthorized,
            Some(String::from(
                "You do not have access to perform this action.",
            )),
        ));
    }

    db_update_user(user, user_id, state)
}
