use crate::ServerState;
use diesel::prelude::*;
use domain::models::review::Review;
use rocket::http::Status;
use uuid::Uuid;

pub fn db_does_unit_exist(
    unit_code: &String,
    state: &ServerState,
) -> Result<bool, (Status, Option<String>)> {
    use domain::schema::units;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        units::table
            .select(units::all_columns)
            .filter(units::unit_code.eq(&unit_code))
            .count()
            .load::<i64>(c)
    }) {
        Ok(unit_count) => {
            if unit_count[0] == 0 {
                return Ok(false);
            }

            Ok(true)
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err((Status::NotFound, Some(String::from("Unit does not exist."))));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn db_read_user_reviews(
    user_id: Uuid,
    state: &ServerState,
) -> Result<Vec<Review>, (Status, Option<String>)> {
    use domain::schema::reviews::{self, user_id as db_user_id};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        reviews::table
            .select(reviews::all_columns)
            .filter(db_user_id.eq(user_id))
            .load::<Review>(c)
    }) {
        Ok(reviews) => {
            return Ok(reviews);
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn db_read_review(
    review_id: i32,
    state: &ServerState,
) -> Result<Review, (Status, Option<String>)> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| reviews::table.find(review_id).first::<Review>(c)) {
        Ok(review) => {
            return Ok(review);
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err((
                    Status::NotFound,
                    Some(String::from("Review could not be found")),
                ));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn db_read_reviews_paginated(
    _page: i64,
    _limit: i64,
    state: &ServerState,
) -> Result<Vec<Review>, (Status, Option<String>)> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        reviews::table
            .limit(_limit)
            .offset(_page * _limit)
            .load::<Review>(c)
    }) {
        Ok(reviews) => Ok(reviews),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn db_read_reviews(state: &ServerState) -> Result<Vec<Review>, (Status, Option<String>)> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| reviews::table.load::<Review>(c)) {
        Ok(reviews) => Ok(reviews),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn db_read_unit_reviews_paginated(
    unit_code: String,
    _page: i64,
    _limit: i64,
    state: &ServerState,
) -> Result<Vec<Review>, (Status, Option<String>)> {
    use domain::schema::reviews::{self, unit_code as db_unit_code};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        reviews::table
            .select(reviews::all_columns)
            .filter(db_unit_code.eq(unit_code))
            .limit(_limit)
            .offset(_page * _limit)
            .load::<Review>(c)
    }) {
        Ok(reviews) => Ok(reviews),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn db_read_unit_reviews(
    unit_code: String,
    state: &ServerState,
) -> Result<Vec<Review>, (Status, Option<String>)> {
    use domain::schema::reviews::{self, unit_code as db_unit_code};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        reviews::table
            .select(reviews::all_columns)
            .filter(db_unit_code.eq(unit_code))
            .load::<Review>(c)
    }) {
        Ok(reviews) => Ok(reviews),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
