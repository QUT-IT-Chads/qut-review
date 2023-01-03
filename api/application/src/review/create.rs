use diesel::prelude::*;
use domain::models::review::{NewReview, NewReviewWithId, Review};
use infrastructure::ServerState;
use rocket::{http::Status, response::status::Created, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};

pub fn create_review(
    review: Json<NewReview>,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::reviews;
    use domain::schema::units;

    let pooled = &mut state.db_pool.get().unwrap();

    let review = review.into_inner();
    let review = NewReviewWithId::new(token.claims.sub, review);

    // Check if unit being review exists
    let unit_count: i64 = match pooled.transaction(|c| {
        units::table
            .select(units::all_columns)
            .filter(units::unit_code.eq(&review.unit_code))
            .count()
            .load(c)
    }) {
        Ok(unit_count) => unit_count[0],
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: Some(String::from("Unit does not exist.")),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if unit_count == 0 {
        let response = ResponseMessage {
            message: Some(String::from("Unit does not exist.")),
        };

        return Err((Status::NotFound, Json(response)));
    }

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
