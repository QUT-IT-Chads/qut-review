use crate::ServerState;
use diesel::prelude::*;
use domain::models::unit::Unit;
use rocket::http::Status;

pub fn db_update_unit(
    unit_code: &str,
    unit: Unit,
    state: &ServerState,
) -> Result<Unit, (Status, Option<String>)> {
    use domain::schema::units::dsl::units;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        diesel::update(units.find(unit_code))
            .set(unit)
            .get_result::<Unit>(c)
    }) {
        Ok(unit) => Ok(unit),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err((
                    Status::NotFound,
                    Some(format!("The unit {} could not be found", unit_code)),
                ));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
