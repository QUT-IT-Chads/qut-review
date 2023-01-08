use crate::ServerState;
use diesel::prelude::*;
use rocket::http::Status;

pub fn db_delete_review(
    review_id: i32,
    state: &ServerState,
) -> Result<Option<String>, (Status, Option<String>)> {
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| diesel::delete(reviews.filter(id.eq(review_id))).execute(c)) {
        Ok(affected_count) => {
            if affected_count > 0 {
                return Ok(None);
            } else {
                return Err((
                    Status::NotFound,
                    Some(String::from("Review could not be found")),
                ));
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
