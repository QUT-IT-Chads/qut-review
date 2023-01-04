use diesel::prelude::*;
use diesel::Connection;
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::State;
use shared::response_models::ResponseMessage;
use shared::token::JWT;

use crate::auth::has_admin_permissions;
use crate::auth::has_user_permissions;

pub fn approve_review(
    review_id: i32,
    status: bool,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Json<Review>, (Status, Json<ResponseMessage>)> {
    use domain::schema::reviews::dsl::*;

    if let Err(err) = has_admin_permissions(&token) {
        return Err(err);
    }

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        diesel::update(reviews.find(review_id))
            .set(approved.eq(status))
            .get_result::<Review>(c)
    }) {
        Ok(review) => {
            return Ok(Json(review));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: Some(format!("The review with ID {} not found", review_id)),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn update_review(
    review_id: i32,
    review: Json<NewReview>,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::reviews;
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();
    let review = review.into_inner();

    let db_review: Review =
        match pooled.transaction(|c| reviews::table.find(review_id).first::<Review>(c)) {
            Ok(review) => review,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = ResponseMessage {
                        message: Some(format!("The review with ID {} not found", review_id)),
                    };

                    return Err((Status::NotFound, Json(response)));
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

    if let Err(err) = has_user_permissions(&token, &db_review.user_id) {
        return Err(err);
    }

    match pooled.transaction(move |c| {
        diesel::update(reviews.find(review_id))
            .set(review)
            .get_result::<Review>(c)
    }) {
        Ok(review) => {
            return Ok(Created::new("")
                .tagged_body(serde_json::to_string(&review).expect("500 internal server error")));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: Some(format!("The review with ID {} not found", review_id)),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
