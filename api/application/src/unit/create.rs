use diesel::prelude::*;
use domain::models::unit::Unit;
use infrastructure::ServerState;
use rocket::{response::status::Created, serde::json::Json, State};

pub fn create_unit(unit: Json<Unit>, state: &State<ServerState>) -> Created<String> {
    use domain::schema::units;

    let unit = unit.into_inner();

    let pooled = &mut state.db_pool.get().unwrap();

    let unit = match pooled.transaction(move |c| {
        diesel::insert_into(units::table)
            .values(&unit)
            .get_result::<Unit>(c)
    }) {
        Ok(reviews) => reviews,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    Created::new("")
        .tagged_body(serde_json::to_string(&unit).expect("Return 500 internal server error."))
}
