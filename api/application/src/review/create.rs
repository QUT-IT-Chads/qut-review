use crate::token::JWT;
use domain::models::review::{NewReview, Review};
use infrastructure::review::create::{db_has_user_reviewed_unit, db_insert_review};
use infrastructure::review::read::db_does_unit_exist;
use infrastructure::ServerState;
use rocket::http::Status;

use crate::auth::has_user_permissions;

pub fn create_review(
    review: NewReview,
    state: &ServerState,
    token: JWT,
) -> Result<Review, (Status, Option<String>)> {
    has_user_permissions(&token, &review.user_id)?;

    let unit_exists = db_does_unit_exist(&review.unit_code, &state)?;

    if !unit_exists {
        return Err((Status::NotFound, Some(String::from("Unit does not exist."))));
    }

    let user_reviewed_unit = db_has_user_reviewed_unit(&review.unit_code, &review.user_id, &state)?;

    if user_reviewed_unit {
        return Err((
            Status::Conflict,
            Some(String::from("Account has already review the desired unit")),
        ));
    }

    db_insert_review(review, &state)
}
