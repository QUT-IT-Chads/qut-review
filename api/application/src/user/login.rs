use diesel::prelude::*;
use domain::models::user::{GetUser, User, LoginRequest};
use infrastructure::ServerState;
use rocket::{response::status::Unauthorized, serde::json::Json, State};
use shared::{
    response_models::{AuthToken, ResponseMessage},
    token::create_jwt,
};

pub fn login_user(
    user: Json<LoginRequest>,
    state: &State<ServerState>,
) -> Result<Json<AuthToken>, Unauthorized<Json<ResponseMessage>>> {
    use domain::schema::users;

    let pooled = &mut state.db_pool.get().unwrap();
    let user = user.into_inner();

    let user: GetUser = match pooled.transaction(move |c| {
        users::table
            .select(users::all_columns)
            .filter(users::email.eq(&user.email))
            .filter(users::hashed_password.eq(&user.hashed_password))
            .first::<User>(c)
    }) {
        Ok(user) => user.into(),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: Some(String::from("Invalid credentials")),
                };

                return Err(Unauthorized(Some(Json(response))));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    match create_jwt(user.id, user.role) {
        Ok(token) => Ok(Json(AuthToken { token })),
        Err(err) => panic!("Error: Failed to create token for user - {}", err),
    }
}
