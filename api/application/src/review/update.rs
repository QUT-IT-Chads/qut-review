use crate::token::JWT;
use domain::models::review::{NewReview, Review};
use infrastructure::review::read::db_read_review;
use infrastructure::review::update::db_update_review;
use infrastructure::review::update::db_update_review_status;
use infrastructure::ServerState;
use rocket::http::Status;

use crate::auth::has_admin_permissions;
use crate::auth::has_user_permissions;

pub fn approve_review(
    review_id: i32,
    status: bool,
    state: &ServerState,
    token: JWT,
) -> Result<Review, (Status, Option<String>)> {
    has_admin_permissions(&token)?;

    db_update_review_status(status, review_id, state)
}

pub fn update_review(
    review_id: i32,
    review: NewReview,
    state: &ServerState,
    token: JWT,
) -> Result<Review, (Status, Option<String>)> {
    let db_review = db_read_review(review_id, state)?;

    has_user_permissions(&token, &db_review.user_id)?;

    db_update_review(review_id, review, state)
}
