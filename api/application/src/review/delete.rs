use diesel::prelude::*;
use domain::models::review::Review;
use infrastructure::ServerState;
use rocket::{http::Status, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};

use crate::auth::has_user_permissions;

pub fn delete_review(
    review_id: i32,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Json<ResponseMessage>, (Status, Json<ResponseMessage>)> {
    use domain::schema::reviews;
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();

    let review: Review =
        match pooled.transaction(|c| reviews::table.find(review_id).first::<Review>(c)) {
            Ok(review) => review,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = ResponseMessage {
                        message: Some(String::from("Review could not be found")),
                    };

                    return Err((Status::NotFound, Json(response)));
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

    if let Err(err) = has_user_permissions(&token, &review.user_id) {
        return Err(err);
    }

    match pooled.transaction(move |c| diesel::delete(reviews.filter(id.eq(review_id))).execute(c)) {
        Ok(affected_count) => {
            if affected_count > 0 {
                let response = ResponseMessage { message: None };

                return Ok(Json(response));
            } else {
                let response = ResponseMessage {
                    message: Some(String::from("Review could not be found")),
                };

                return Err((Status::NotFound, Json(response)));
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
