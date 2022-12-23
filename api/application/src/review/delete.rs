use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_review(review_id: u32) -> Result<String, NotFound<String>> {
    let response = Response {
        body: ResponseBody::Message(format!("Error: review with ID {} not found", review_id)),
    };

    Err(NotFound(
        serde_json::to_string(&response).expect("Return 500 internal server error."),
    ))
}
