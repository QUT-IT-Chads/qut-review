use diesel::prelude::*;
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::{http::Status, response::status::Created, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};

use crate::auth::{has_user_permissions, unit_exists};

pub fn create_review(
    review: Json<NewReview>,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::reviews;

    let review = review.into_inner();

    if let Err(err) = has_user_permissions(&token, &review.user_id) {
        return Err(err);
    }

    let pooled = &mut state.db_pool.get().unwrap();

    if let Err(err) = unit_exists(&review.unit_code, pooled) {
        return Err(err);
    };

    // Check if user has already reviewed the unit
    let review_count: i64 = match pooled.transaction(|c| {
        reviews::table
            .select(reviews::all_columns)
            .filter(reviews::unit_code.eq(&review.unit_code))
            .filter(reviews::user_id.eq(&review.user_id))
            .count()
            .load(c)
    }) {
        Ok(review_count) => review_count[0],
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if review_count > 0 {
        let response = ResponseMessage {
            message: Some(String::from("Account has already review the desired unit")),
        };
        return Err((Status::Conflict, Json(response)));
    }

    let review = match pooled.transaction(|c| {
        diesel::insert_into(reviews::table)
            .values(&review)
            .get_result::<Review>(c)
    }) {
        Ok(reviews) => reviews,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    Ok(Created::new("")
        .tagged_body(serde_json::to_string(&review).expect("Return 500 internal server error.")))
}
