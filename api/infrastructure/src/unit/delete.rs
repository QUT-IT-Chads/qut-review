use crate::ServerState;
use diesel::prelude::*;
use rocket::http::Status;

pub fn db_delete_unit(
    unit_code: &str,
    state: &ServerState,
) -> Result<Option<String>, (Status, Option<String>)> {
    use domain::schema::units::dsl::{unit_code as db_unit_code, units};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled
        .transaction(move |c| diesel::delete(units.filter(db_unit_code.eq(unit_code))).execute(c))
    {
        Ok(affected_count) => {
            if affected_count > 0 {
                return Ok(None);
            } else {
                return Err((
                    Status::NotFound,
                    Some(format!("The unit '{}' could not found", unit_code)),
                ));
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
