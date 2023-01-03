use diesel::prelude::*;
use domain::enums::role::Role;
use infrastructure::ServerState;
use rocket::{http::Status, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};

pub fn delete_unit(
    unit_code: &str,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Json<ResponseMessage>, (Status, Json<ResponseMessage>)> {
    use domain::schema::units::dsl::{unit_code as db_unit_code, units};

    if token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from("You do not have access to perform this action.")),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled
        .transaction(move |c| diesel::delete(units.filter(db_unit_code.eq(unit_code))).execute(c))
    {
        Ok(affected_count) => {
            if affected_count > 0 {
                let response = ResponseMessage {
                    message: None,
                };

                return Ok(Json(response));
            } else {
                let response = ResponseMessage {
                    message: Some(format!("The unit '{}' could not found", unit_code)),
                };

                return Err((Status::NotFound, Json(response)));
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
