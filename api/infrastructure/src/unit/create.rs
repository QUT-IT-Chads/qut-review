use crate::ServerState;
use diesel::prelude::*;
use domain::models::unit::Unit;
use rocket::http::Status;

pub fn db_insert_unit(unit: Unit, state: &ServerState) -> Result<Unit, (Status, Option<String>)> {
    use domain::schema::units;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        diesel::insert_into(units::table)
            .values(&unit)
            .get_result::<Unit>(c)
    }) {
        Ok(unit) => Ok(unit),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
