use crate::token::JWT;
use infrastructure::{
    review::{delete::db_delete_review, read::db_read_review},
    ServerState,
};
use rocket::http::Status;

use crate::auth::has_user_permissions;

pub fn delete_review(
    review_id: i32,
    state: &ServerState,
    token: JWT,
) -> Result<Option<String>, (Status, Option<String>)> {
    let review = db_read_review(review_id, state)?;

    has_user_permissions(&token, &review.user_id)?;

    db_delete_review(review_id, state)
}
