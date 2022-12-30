use diesel::prelude::*;
use domain::{models::user::{GetUser, User}, enums::role::Role};
use infrastructure::ServerState;
use rocket::{http::Status, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};
use uuid::Uuid;

pub fn list_user(
    user_id: Uuid,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Json<GetUser>, (Status, Json<ResponseMessage>)> {
    use domain::schema::users;

    let pooled = &mut state.db_pool.get().unwrap();

    if token.claims.sub != user_id && token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: (String::from("You do not have access to perform this action.")),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    match pooled.transaction(move |c| users::table.find(user_id).first::<User>(c)) {
        Ok(user) => {
            // Converting User to GetUser which removes password
            return Ok(Json(user.into()));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: (format!("Error: user with user id {} not found - {}", user_id, err)),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
