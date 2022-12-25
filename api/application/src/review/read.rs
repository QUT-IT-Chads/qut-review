use diesel::prelude::*;
use domain::models::review::Review;
use infrastructure::ServerState;
use rocket::{response::status::NotFound, State, serde::json::Json};
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub fn list_review(review_id: i32, state: &State<ServerState>) -> Result<Json<Response>, NotFound<Json<Response>>> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| reviews::table.find(review_id).first::<Review>(c)) {
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

pub fn list_user_reviews(
    user_id: Uuid,
    state: &State<ServerState>,
) -> Result<Json<Response>, NotFound<Json<Response>>> {
    use domain::schema::reviews::{self, user_id as db_user_id};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        reviews::table
            .select(reviews::all_columns)
            .filter(db_user_id.eq(user_id))
            .load::<Review>(c)
    }) {
        Ok(reviews) => {
            let response = Response {
                body: ResponseBody::Reviews(reviews),
            };

            return Ok(Json(response));
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
