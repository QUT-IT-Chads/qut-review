use diesel::prelude::*;
use diesel::Connection;
use domain::models::unit::Unit;
use infrastructure::ServerState;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::State;
use shared::response_models::ResponseMessage;

pub fn update_unit(
    unit_code: &str,
    unit: Json<Unit>,
    state: &State<ServerState>,
) -> Result<Created<String>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::units::dsl::{units, unit_code as db_unit_code};

    let pooled = &mut state.db_pool.get().unwrap();
    let unit = unit.into_inner();

    match pooled.transaction(move |c| {
        diesel::update(units.find(unit_code))
            .set(unit)
            .get_result::<Unit>(c)
    }) {
        Ok(review) => {
            return Ok(Created::new("")
                .tagged_body(serde_json::to_string(&review).expect("500 internal server error")));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: format!("Error: unit with unit code {} not found - {}", unit_code, err),
                };

                return Err(NotFound(Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
