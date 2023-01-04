use diesel::prelude::*;
use infrastructure::ServerState;
use rocket::{http::Status, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};
use uuid::Uuid;

use crate::auth::has_user_permissions;

pub fn delete_user(
    user_id: Uuid,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Json<ResponseMessage>, (Status, Json<ResponseMessage>)> {
    use domain::schema::users::dsl::{id as db_user_id, users};

    if let Err(err) = has_user_permissions(&token, &user_id) {
        return Err(err);
    }

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled
        .transaction(move |c| diesel::delete(users.filter(db_user_id.eq(user_id))).execute(c))
    {
        Ok(affected_count) => {
            if affected_count > 0 {
                let response = ResponseMessage { message: None };

                return Ok(Json(response));
            } else {
                let response = ResponseMessage {
                    message: Some(String::from("User not found")),
                };

                return Err((Status::NotFound, Json(response)));
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
