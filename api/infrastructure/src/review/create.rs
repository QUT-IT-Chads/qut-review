use crate::ServerState;
use diesel::prelude::*;
use domain::models::review::{NewReview, Review};
use rocket::http::Status;
use uuid::Uuid;

pub fn db_has_user_reviewed_unit(
    unit_code: &str,
    user_id: &Uuid,
    state: &ServerState,
) -> Result<bool, (Status, Option<String>)> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    let review_count = pooled
        .transaction(move |c| {
            reviews::table
                .select(reviews::all_columns)
                .filter(reviews::unit_code.eq(&unit_code))
                .filter(reviews::user_id.eq(&user_id))
                .count()
                .load::<i64>(c)
        })
        .expect("Database error");

    Ok(review_count[0] > 0)
}

pub fn db_insert_review(
    review: NewReview,
    state: &ServerState,
) -> Result<Review, (Status, Option<String>)> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    let review = pooled
        .transaction(move |c| {
            diesel::insert_into(reviews::table)
                .values(&review)
                .get_result::<Review>(c)
        })
        .expect("Database error");

    Ok(review)
}
