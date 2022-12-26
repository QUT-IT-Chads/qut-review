use diesel::prelude::*;
use domain::models::user::{NewUser, User};
use infrastructure::ServerState;
use rocket::{response::status::Created, serde::json::Json, State};
use uuid::Uuid;

pub fn create_user(user: Json<NewUser>, state: &State<ServerState>) -> Created<String> {
    use domain::schema::users;

    let user = user.into_inner();
    let id = Uuid::new_v4();
    let user = User::new(id, user);

    let pooled = &mut state.db_pool.get().unwrap();

    let user = match pooled.transaction(move |c| {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(c)
    }) {
        Ok(reviews) => reviews,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    Created::new("")
        .tagged_body(serde_json::to_string(&user).expect("Return 500 internal server error."))
}
