use crate::ServerState;
use diesel::prelude::*;
use domain::models::review::{NewReview, Review};
use rocket::http::Status;

pub fn db_update_review_status(
    status: bool,
    review_id: i32,
    state: &ServerState,
) -> Result<Review, (Status, Option<String>)> {
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        diesel::update(reviews.find(review_id))
            .set(approved.eq(status))
            .get_result::<Review>(c)
    }) {
        Ok(review) => Ok(review),
        Err(diesel::result::Error::NotFound) => Err((
            Status::NotFound,
            Some(format!("The review with ID {} not found", review_id)),
        )),
        Err(err) => panic!("Database error - {}", err),
    }
}

pub fn db_update_review(
    review_id: i32,
    review: NewReview,
    state: &ServerState,
) -> Result<Review, (Status, Option<String>)> {
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        diesel::update(reviews.find(review_id))
            .set(review)
            .get_result::<Review>(c)
    }) {
        Ok(review) => Ok(review),
        Err(diesel::result::Error::NotFound) => Err((
            Status::NotFound,
            Some(format!("The review with ID {} not found", review_id)),
        )),
        Err(err) => panic!("Database error - {}", err),
    }
}
