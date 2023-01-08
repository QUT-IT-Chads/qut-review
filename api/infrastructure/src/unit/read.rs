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
        Ok(unit) => {
            return Ok(unit);
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Some(format!("The unit '{}' could not found", unit_code));

                return Err((Status::NotFound, response));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn db_read_units(state: &ServerState) -> Result<Vec<Unit>, (Status, Option<String>)> {
    use domain::schema::units;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| units::table.load::<Unit>(c)) {
        Ok(units) => Ok(units),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
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
