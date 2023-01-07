use crate::ServerState;
use diesel::prelude::*;
use domain::models::user::{GetUser, UpdateUser, User};
use rocket::{http::Status, State};
use uuid::Uuid;

pub fn db_update_user(
    user: UpdateUser,
    user_id: Uuid,
    state: &State<ServerState>,
) -> Result<GetUser, (Status, Option<String>)> {
    use domain::schema::users::dsl::users;

    let pooled = &mut state.db_pool.get().unwrap();
    match pooled.transaction(move |c| {
        diesel::update(users.find(user_id))
            .set(user)
            .get_result::<User>(c)
    }) {
        Ok(user) => Ok(user.get_public()),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err((
                    Status::NotFound,
                    Some(String::from("User could not be found")),
                ));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
