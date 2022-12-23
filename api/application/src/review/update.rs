use domain::models::review::UpdateReview;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn approve_review(review_id: u32, status: bool) -> Result<Created<String>, NotFound<String>> {
    let response = Response {
        body: ResponseBody::Message(format!("Error: review with ID {} not found", review_id)),
    };

    Err(NotFound(
        serde_json::to_string(&response).expect("Return 500 internal server error."),
    ))
}

pub fn update_review(
    review_id: u32,
    review: Json<UpdateReview>,
) -> Result<Created<String>, NotFound<String>> {
    let review = review.into_inner();

    let response = Response {
        body: ResponseBody::Message(format!("Error: review with ID {} not found", review_id)),
    };

    Err(NotFound(
        serde_json::to_string(&response).expect("Return 500 internal server error."),
    ))
}
