use diesel::prelude::*;
use domain::models::review::Review;
use infrastructure::ServerState;
use rocket::{response::status::NotFound, State};
use shared::response_models::{Response, ResponseBody};

use crate::serializer::serialize_response;

pub fn list_review(review_id: i32, state: &State<ServerState>) -> Result<String, NotFound<String>> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| reviews::table.find(review_id).first::<Review>(c)) {
        Ok(review) => {
            let response = Response {
                body: ResponseBody::Review(review),
            };

            return Ok(serialize_response(response));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error: review with ID {} not found - {}",
                        review_id, err
                    )),
                };

                return Err(NotFound(serialize_response(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_reviews(state: &State<ServerState>) -> Vec<Review> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| reviews::table.load::<Review>(c)) {
        Ok(reviews) => reviews,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
