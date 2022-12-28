use diesel::prelude::*;
use domain::models::user::{User, GetUser};
use infrastructure::ServerState;
use rocket::{response::status::NotFound, serde::json::Json, State};
use shared::response_models::ResponseMessage;
use uuid::Uuid;

pub fn list_user(
    user_id: Uuid,
    state: &State<ServerState>,
) -> Result<Json<GetUser>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::users;

    let pooled = &mut state.db_pool.get().unwrap();

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

                return Err(NotFound(Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
