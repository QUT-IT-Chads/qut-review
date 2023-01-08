use domain::models::review::Review;
use infrastructure::{
    review::read::{
        db_read_review, db_read_reviews, db_read_reviews_paginated, db_read_unit_reviews,
        db_read_unit_reviews_paginated, db_read_user_reviews,
    },
    ServerState,
};
use rocket::http::Status;
use uuid::Uuid;

pub fn list_review(
    review_id: i32,
    state: &ServerState,
) -> Result<Review, (Status, Option<String>)> {
    db_read_review(review_id, state)
}

pub fn list_reviews(
    _page: Option<i64>,
    _limit: Option<i64>,
    state: &ServerState,
) -> Result<Vec<Review>, (Status, Option<String>)> {
    // Return paginated reviews
    if let (Some(page), Some(limit)) = (_page, _limit) {
        return db_read_reviews_paginated(page, limit, state);
    }

    // Return all reviews
    db_read_reviews(state)
}

pub fn list_unit_reviews(
    unit_code: String,
    _page: Option<i64>,
    _limit: Option<i64>,
    state: &ServerState,
) -> Result<Vec<Review>, (Status, Option<String>)> {
    // Return paginated reviews
    if let (Some(page), Some(limit)) = (_page, _limit) {
        return db_read_unit_reviews_paginated(unit_code, page, limit, state);
    }

    // Return all reviews
    db_read_unit_reviews(unit_code, state)
}

pub fn list_user_reviews(
    user_id: Uuid,
    state: &ServerState,
) -> Result<Vec<Review>, (Status, Option<String>)> {
    db_read_user_reviews(user_id, state)
}
