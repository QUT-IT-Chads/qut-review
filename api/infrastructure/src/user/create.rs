use crate::ServerState;
use diesel::prelude::*;
use domain::models::user::{GetUser, User};
use rocket::http::Status;

pub fn db_insert_user(
    user: User,
    state: &ServerState,
) -> Result<GetUser, (Status, Option<String>)> {
    use domain::schema::users;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(|c| {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(c)
    }) {
        Ok(user) => Ok(user.get_public()),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
