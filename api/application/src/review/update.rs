use diesel::prelude::*;
use diesel::Connection;
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::State;
use shared::response_models::ResponseMessage;

pub fn approve_review(
    review_id: i32,
    status: bool,
    state: &State<ServerState>,
) -> Result<Json<Review>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::reviews::dsl::*;

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
                    message: format!("Error: review with ID {} not found - {}", review_id, err),
                };

                return Err(NotFound(Json(response)));
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
) -> Result<Created<String>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();
    let review = review.into_inner();

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
                    message: format!("Error: review with ID {} not found - {}", review_id, err),
                };

                return Err(NotFound(Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
