use crate::ServerState;
use diesel::prelude::*;
use rocket::http::Status;
use uuid::Uuid;

pub fn db_delete_user(
    user_id: &Uuid,
    state: &ServerState,
) -> Result<Option<String>, (Status, Option<String>)> {
    use domain::schema::users::dsl::{id as db_user_id, users};

    let pooled = &mut state.db_pool.get().unwrap();

    let affected_count = pooled
        .transaction(move |c| diesel::delete(users.filter(db_user_id.eq(user_id))).execute(c))
        .expect("Database error");

    if affected_count > 0 {
        Ok(None)
    } else {
        Err((Status::NotFound, Some(String::from("User not found"))))
    }
}