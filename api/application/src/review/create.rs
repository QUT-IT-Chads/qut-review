use domain::models::review::{NewReview, Review};
use rocket::{response::status::Created, serde::json::Json};
use shared::response_models::{Response, ResponseBody};

pub fn create_review(review: Json<NewReview>) -> Created<String> {
    let review = review.into_inner();

    // Database should give back this value
    let review = Review::new(review);

    let response = Response { body: ResponseBody::Review(review)};
    Created::new("").tagged_body(serde_json::to_string(&response).expect("Return 500 internal server error."))
}
