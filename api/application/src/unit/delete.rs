use diesel::prelude::*;
use infrastructure::ServerState;
use rocket::{response::status::NotFound, serde::json::Json, State};
use shared::response_models::ResponseMessage;

pub fn delete_unit(
    unit_code: &str,
    state: &State<ServerState>,
) -> Result<Json<ResponseMessage>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::units::dsl::{units, unit_code as db_unit_code};

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| diesel::delete(units.filter(db_unit_code.eq(unit_code))).execute(c)) {
        Ok(affected_count) => {
            if affected_count > 0 {
                let response = ResponseMessage {
                    message: format!("Successfully deleted unit with unit code {}", unit_code),
                };

                return Ok(Json(response));
            } else {
                let response = ResponseMessage {
                    message: format!("Error: unit with unit code {} not found", unit_code),
                };

                return Err(NotFound(Json(response)));
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
