use diesel::prelude::*;
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::{response::status::Created, serde::json::Json, State};
use shared::response_models::{Response, ResponseBody};

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

    let response = Response {
        body: ResponseBody::Review(review),
    };
    Created::new("")
        .tagged_body(serde_json::to_string(&response).expect("Return 500 internal server error."))
}
