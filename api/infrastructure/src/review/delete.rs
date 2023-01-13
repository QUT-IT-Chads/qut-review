use crate::ServerState;
use diesel::prelude::*;
use rocket::http::Status;

pub fn db_delete_review(
    review_id: i32,
    state: &ServerState,
) -> Result<Option<String>, (Status, Option<String>)> {
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();

    let affected_count = pooled
        .transaction(move |c| diesel::delete(reviews.filter(id.eq(review_id))).execute(c))
        .expect("Database error");

    if affected_count > 0 {
        Ok(None)
    } else {
        Err((
            Status::NotFound,
            Some(String::from("Review could not be found")),
        ))
    }
}
