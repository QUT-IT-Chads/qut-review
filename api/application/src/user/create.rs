use domain::models::user::{GetUser, LoginRequest, User};
use infrastructure::ServerState;
use rocket::{http::Status, State};
use uuid::Uuid;

use infrastructure::user::create::db_insert_user;
use infrastructure::user::read::db_does_user_exist;

pub fn create_user(
    user: LoginRequest,
    state: &State<ServerState>,
) -> Result<GetUser, (Status, Option<String>)> {
    let id = Uuid::new_v4();
    let user = User::new(id, user.into());

    match db_does_user_exist(&user.get_public().email, &state) {
        Ok(exists) => {
            if exists {
                return Err((
                    Status::Conflict,
                    Some(String::from("Email is already in use")),
                ));
            }
        }
        Err(err) => return Err(err),
    };

    db_insert_user(user, &state)
}
