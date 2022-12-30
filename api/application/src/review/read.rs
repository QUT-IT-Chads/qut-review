use diesel::prelude::*;
use domain::models::review::Review;
use infrastructure::ServerState;
use rocket::{response::status::NotFound, serde::json::Json, State};
use shared::response_models::ResponseMessage;
use uuid::Uuid;

pub fn list_review(
    review_id: i32,
    state: &State<ServerState>,
) -> Result<Json<Review>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| reviews::table.find(review_id).first::<Review>(c)) {
        Ok(review) => {
            return Ok(Json(review));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: Some(String::from("Review could not be found")),
                };

                return Err(NotFound(Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_reviews(
    _page: Option<i64>,
    _limit: Option<i64>,
    state: &State<ServerState>,
) -> Json<Vec<Review>> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        // Return paginated results
        if let (Some(page), Some(limit)) = (_page, _limit) {
            return reviews::table
                .limit(limit)
                .offset(page * limit)
                .load::<Review>(c);
        }

        // Return all reviews
        reviews::table.load::<Review>(c)
    }) {
        Ok(reviews) => Json(reviews),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_unit_reviews(
    unit_code: String,
    _page: Option<i64>,
    _limit: Option<i64>,
    state: &State<ServerState>,
) -> Json<Vec<Review>> {
    use domain::schema::reviews::{self, unit_code as db_unit_code};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        // Return paginated results
        if let (Some(page), Some(limit)) = (_page, _limit) {
            return reviews::table
                .select(reviews::all_columns)
                .filter(db_unit_code.eq(unit_code))
                .limit(limit)
                .offset(page * limit)
                .load::<Review>(c);
        }

        // Return all reviews by unit
        reviews::table
            .select(reviews::all_columns)
            .filter(db_unit_code.eq(unit_code))
            .load::<Review>(c)
    }) {
        Ok(reviews) => {
            return Json(reviews);
        }
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
) -> Result<Json<Vec<Review>>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::reviews::{self, user_id as db_user_id};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        reviews::table
            .select(reviews::all_columns)
            .filter(db_user_id.eq(user_id))
            .load::<Review>(c)
    }) {
        Ok(reviews) => {
            return Ok(Json(reviews));
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
