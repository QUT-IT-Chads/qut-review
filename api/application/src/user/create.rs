use diesel::prelude::*;
use domain::models::user::{GetUser, LoginRequest, User};
use infrastructure::ServerState;
use rocket::{http::Status, response::status::Created, serde::json::Json, State};
use shared::response_models::ResponseMessage;
use uuid::Uuid;

use crate::database_helpers::user_exists;

pub fn create_user(
    user: Json<LoginRequest>,
    state: &State<ServerState>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::users;

    let user = user.into_inner();

    let id = Uuid::new_v4();
    let user = User::new(id, user.into());

    let pooled = &mut state.db_pool.get().unwrap();

    match user_exists(&user.get_public().email, pooled) {
        Ok(exists) => {
            if exists {
                let response = ResponseMessage {
                    message: Some(String::from("Email is already in use")),
                };

                return Err((Status::Conflict, Json(response)));
            }
        }
        Err(err) => return Err(err),
    };

    let user: GetUser = match pooled.transaction(|c| {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(c)
    }) {
        Ok(user) => user.get_public(),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    Ok(Created::new("")
        .tagged_body(serde_json::to_string(&user).expect("Return 500 internal server error.")))
}
