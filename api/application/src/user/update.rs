use diesel::prelude::*;
use diesel::Connection;
use domain::models::user::User;
use infrastructure::ServerState;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::State;
use shared::response_models::ResponseMessage;
use uuid::Uuid;

pub fn update_user(
    user_id: Uuid,
    user: Json<User>,
    state: &State<ServerState>,
) -> Result<Created<String>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::users::dsl::users;

    let pooled = &mut state.db_pool.get().unwrap();
    let user = user.into_inner();

    match pooled.transaction(move |c| {
        diesel::update(users.find(user_id))
            .set(user)
            .get_result::<User>(c)
    }) {
        Ok(review) => {
            return Ok(Created::new("")
                .tagged_body(serde_json::to_string(&review).expect("500 internal server error")));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: format!(
                        "Error: user with user id {} not found - {}",
                        user_id, err
                    ),
                };

                return Err(NotFound(Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
