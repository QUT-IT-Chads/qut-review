use shared::response_models::{Response, ResponseBody};
use application::review::{read};
use domain::models::{Review};
use rocket::{get};
use rocket::response::status::{NotFound};

/// Returns a JSON serialized vector of all reviews
#[get("/")]
pub fn list_reviews_handler() -> String {
    let reviews: Vec<Review> = read::list_reviews();
    let response = Response { body: ResponseBody::Reviews(reviews) };

    serde_json::to_string(&response).expect("Return 500 internal server error.")
}

/// Takes in a `review_id` and returns the associated review as a JSON serialized review otherwise a NotFound NetworkResponse.
#[get("/<review_id>")]
pub fn list_review_handler(review_id: u32) -> Result<String, NotFound<String>> {
    let review: Review = read::list_review(review_id)?;
    let response = Response { body: ResponseBody::Review(review) };

    Ok(serde_json::to_string(&response).expect("Return 500 internal server error."))
}
