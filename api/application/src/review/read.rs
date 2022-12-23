use domain::models::review::Review;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn list_review(review_id: u32) -> Result<Review, NotFound<String>> {
    let response = Response {
        body: ResponseBody::Message(format!("Error: review with ID {} not found", review_id)),
    };
    Err(NotFound(
        serde_json::to_string(&response).expect("Return 500 internal server error."),
    ))
}

pub fn list_reviews() -> Vec<Review> {
    vec![]
}
