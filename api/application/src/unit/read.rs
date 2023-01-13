use crate::response_models::ResponseMessage;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use domain::models::unit::Unit;
use infrastructure::unit::read::{db_read_unit, db_read_units};
use infrastructure::ServerState;
use rocket::{http::Status, serde::json::Json};

pub fn list_unit(unit_code: &str, state: &ServerState) -> Result<Unit, (Status, Option<String>)> {
    db_read_unit(unit_code, state)
}

pub fn list_units(state: &ServerState) -> Result<Vec<Unit>, (Status, Option<String>)> {
    db_read_units(state)
}

pub fn unit_exists(
    unit_code: &String,
    pooled: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<bool, (Status, Json<ResponseMessage>)> {
    use domain::schema::units;

    match pooled.transaction(move |c| {
        units::table
            .select(units::all_columns)
            .filter(units::unit_code.eq(&unit_code))
            .count()
            .load::<i64>(c)
    }) {
        Ok(unit_count) => Ok(unit_count[0] != 0),
        Err(diesel::result::Error::NotFound) => Err((
            Status::NotFound,
            Json(ResponseMessage {
                message: Some(String::from("Unit does not exist.")),
            }),
        )),
        Err(err) => panic!("Database error - {}", err),
    }
}
