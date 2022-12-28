use diesel::prelude::*;
use domain::models::user::{GetUser, NewUser, User};
use infrastructure::ServerState;
use rocket::{
    response::status::{Conflict, Created},
    serde::json::Json,
    State,
};
use shared::response_models::ResponseMessage;
use uuid::Uuid;

pub fn create_user(
    user: Json<NewUser>,
    state: &State<ServerState>,
) -> Result<Created<String>, Conflict<String>> {
    use domain::schema::users;

    let user = user.into_inner();
    let id = Uuid::new_v4();
    let user = User::new(id, user);

    let pooled = &mut state.db_pool.get().unwrap();

    let user_count: i64 = match pooled.transaction(|c| {
        users::table
            .select(users::all_columns)
            .filter(users::email.eq(&user.email))
            .count()
            .load(c)
    }) {
        Ok(user_count) => user_count[0],
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if user_count > 0 {
        let response = ResponseMessage {
            message: String::from("Email is already in use"),
        };

        return Err(Conflict(Some(
            serde_json::to_string(&response).expect("Return 500 internal server error."),
        )));
    }

    let user: GetUser = match pooled.transaction(|c| {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(c)
    }) {
        Ok(user) => user.into(),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    Ok(Created::new("")
        .tagged_body(serde_json::to_string(&user).expect("Return 500 internal server error.")))
}
