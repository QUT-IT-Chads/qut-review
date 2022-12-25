use diesel::prelude::*;
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::{response::status::Created, serde::json::Json, State};

pub fn create_review(review: Json<NewReview>, state: &State<ServerState>) -> Created<String> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();
    let review = review.into_inner();

    let review = match pooled.transaction(move |c| {
        diesel::insert_into(reviews::table)
            .values(&review)
            .get_result::<Review>(c)
    }) {
        Ok(reviews) => reviews,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    Created::new("")
        .tagged_body(serde_json::to_string(&review).expect("Return 500 internal server error."))
}
