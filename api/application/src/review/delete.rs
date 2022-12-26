use diesel::prelude::*;
use infrastructure::ServerState;
use rocket::{response::status::NotFound, State};
use shared::response_models::{Response, ResponseBody};

use crate::serializer::serialize_response;

pub fn delete_review(
    review_id: i32,
    state: &State<ServerState>,
) -> Result<String, NotFound<String>> {
    use domain::schema::reviews::dsl::*;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| diesel::delete(reviews.filter(id.eq(review_id))).execute(c)) {
        Ok(affected_count) => {
            if affected_count > 0 {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Successfully deleted review with ID {}",
                        review_id
                    )),
                };

                return Ok(serialize_response(response));
            } else {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error: review with ID {} not found",
                        review_id
                    )),
                };

                return Err(NotFound(serialize_response(response)));
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
