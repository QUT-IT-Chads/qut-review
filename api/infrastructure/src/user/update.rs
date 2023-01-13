use crate::ServerState;
use diesel::prelude::*;
use domain::models::user::{GetUser, UpdateUser, User};
use rocket::http::Status;
use uuid::Uuid;

pub fn db_update_user(
    user: UpdateUser,
    user_id: Uuid,
    state: &ServerState,
) -> Result<GetUser, (Status, Option<String>)> {
    use domain::schema::users::dsl::users;

    let pooled = &mut state.db_pool.get().unwrap();
    match pooled.transaction(move |c| {
        diesel::update(users.find(user_id))
            .set(user)
            .get_result::<User>(c)
    }) {
        Ok(user) => Ok(user.get_public()),
        Err(diesel::result::Error::NotFound) => Err((
            Status::NotFound,
            Some(String::from("User could not be found")),
        )),
        Err(err) => panic!("Database error - {}", err),
    }
}
