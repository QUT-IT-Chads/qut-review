use diesel::prelude::*;
use infrastructure::ServerState;
use rocket::{response::status::NotFound, serde::json::Json, State};
use shared::response_models::ResponseMessage;
use uuid::Uuid;

pub fn delete_user(
    user_id: Uuid,
    state: &State<ServerState>,
) -> Result<Json<ResponseMessage>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::users::dsl::{users, id as db_user_id};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| diesel::delete(users.filter(db_user_id.eq(user_id))).execute(c)) {
        Ok(affected_count) => {
            if affected_count > 0 {
                let response = ResponseMessage {
                    message: format!("Successfully deleted user with user id {}", user_id),
                };

                return Ok(Json(response));
            } else {
                let response = ResponseMessage {
                    message: format!("Error: user with user id {} not found", user_id),
                };

                return Err(NotFound(Json(response)));
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
