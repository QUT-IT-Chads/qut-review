use diesel::prelude::*;
use diesel::Connection;
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::State;
use shared::response_models::{Response, ResponseBody};

pub fn approve_review(
    review_id: i32,
    status: bool,
    state: &State<ServerState>,
) -> Result<Json<Response>, NotFound<Json<Response>>> {
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        diesel::update(reviews.find(review_id))
            .set(approved.eq(status))
            .get_result::<Review>(c)
    }) {
        Ok(review) => {
            let response = Response {
                body: ResponseBody::Review(review),
            };

            return Ok(Json(response));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error: review with ID {} not found - {}",
                        review_id, err
                    )),
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
) -> Result<Created<String>, NotFound<Json<Response>>> {
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();
    let review = review.into_inner();

    match pooled.transaction(move |c| {
        diesel::update(reviews.find(review_id))
            .set(review)
            .get_result::<Review>(c)
    }) {
        Ok(review) => {
            let response = Response {
                body: ResponseBody::Review(review),
            };

            return Ok(Created::new("").tagged_body(serde_json::to_string(&response).expect("500 internal server error")));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error: review with ID {} not found - {}",
                        review_id, err
                    )),
                };

                return Err(NotFound(Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
