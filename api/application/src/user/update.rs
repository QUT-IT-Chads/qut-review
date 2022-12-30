use diesel::prelude::*;
use diesel::Connection;
use domain::enums::role::Role;
use domain::models::user::GetUser;
use domain::models::user::{NewUser, User};
use infrastructure::ServerState;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::State;
use shared::response_models::ResponseMessage;
use shared::token::JWT;
use uuid::Uuid;

pub fn update_user(
    user_id: Uuid,
    user: Json<NewUser>,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::users::dsl::users;

    if token.claims.sub != user_id && token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from("You do not have access to perform this action.")),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    let pooled = &mut state.db_pool.get().unwrap();
    let user = user.into_inner();

    if user.role == Role::Admin && token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from("You do not have access to perform this action.")),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    match pooled.transaction(move |c| {
        diesel::update(users.find(user_id))
            .set(user)
            .get_result::<User>(c)
    }) {
        Ok(user) => {
            let user: GetUser = user.into();

            return Ok(Created::new("")
                .tagged_body(serde_json::to_string(&user).expect("500 internal server error")));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: Some(String::from("User could not be found")),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
