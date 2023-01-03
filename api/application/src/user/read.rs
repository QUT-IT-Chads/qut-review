use diesel::prelude::*;
use domain::{
    enums::role::Role,
    models::user::{GetUser, User},
};
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

    if token.claims.sub != user_id && token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from(
                "You do not have access to perform this action.",
            )),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| users::table.find(user_id).first::<User>(c)) {
        Ok(user) => {
            // Converting User to GetUser which removes password
            return Ok(Json(user.into()));
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
