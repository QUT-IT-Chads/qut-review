use domain::models::Review;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};
use crate::dummy_data::{review_dummy_data};

pub fn list_review(review_id: u32) -> Result<Review, NotFound<String>> {
    Ok(review_dummy_data::get_review())
}

pub fn list_reviews() -> Vec<Review> {
    vec![
        review_dummy_data::get_review()
    ]
}
