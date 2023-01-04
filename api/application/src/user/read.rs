use diesel::prelude::*;
use domain::models::user::{GetUser, User};
use infrastructure::ServerState;
use rocket::{http::Status, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};
use uuid::Uuid;

use crate::auth::has_user_permissions;

pub fn list_user(
    user_id: Uuid,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Json<GetUser>, (Status, Json<ResponseMessage>)> {
    use domain::schema::users;

    if let Err(err) = has_user_permissions(&token, &user_id) {
        return Err(err);
    }

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| users::table.find(user_id).first::<User>(c)) {
        Ok(user) => {
            return Ok(Json(user.get_public()));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: Some(String::from("User could not be found.")),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
