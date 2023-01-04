use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use rocket::{http::Status, serde::json::Json};
use shared::response_models::ResponseMessage;

pub fn user_exists(
    user_email: &String,
    pooled: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<bool, (Status, Json<ResponseMessage>)> {
    use domain::schema::users;

    match pooled.transaction(|c| {
        users::table
            .select(users::all_columns)
            .filter(users::email.eq(&user_email))
            .count()
            .load::<i64>(c)
    }) {
        Ok(user_count) => {
            if user_count[0] == 0 {
                return Ok(false);
            }

            Ok(true)
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn unit_exists(
    unit_code: &String,
    pooled: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<bool, (Status, Json<ResponseMessage>)> {
    use domain::schema::units;

    match pooled.transaction(|c| {
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
                let response = ResponseMessage {
                    message: Some(String::from("Unit does not exist.")),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
