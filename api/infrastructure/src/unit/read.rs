use crate::ServerState;
use diesel::prelude::*;
use domain::models::unit::Unit;
use rocket::{http::Status, State};

pub fn db_read_unit(
    unit_code: &str,
    state: &ServerState,
) -> Result<Unit, (Status, Option<String>)> {
    use domain::schema::units;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| units::table.find(unit_code).first::<Unit>(c)) {
        Ok(unit) => Ok(unit),
        Err(diesel::result::Error::NotFound) => Err((
            Status::NotFound,
            Some(format!("The unit '{}' could not found", unit_code)),
        )),
        Err(err) => panic!("Database error - {}", err),
    }
}

pub fn db_read_units(state: &ServerState) -> Result<Vec<Unit>, (Status, Option<String>)> {
    use domain::schema::units;

    let pooled = &mut state.db_pool.get().unwrap();

    let units = pooled
        .transaction(move |c| units::table.load::<Unit>(c))
        .expect("Database error");

    Ok(units)
}

pub fn db_does_unit_exist(
    unit_code: &String,
    state: &State<ServerState>,
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
        Ok(unit_count) => Ok(unit_count[0] != 0),
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, Some(String::from("Unit does not exist.")))),
        Err(err) => panic!("Database error - {}", err),
    }
}
