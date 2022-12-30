use diesel::prelude::*;
use diesel::Connection;
use domain::enums::role::Role;
use domain::models::unit::Unit;
use infrastructure::ServerState;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::State;
use shared::response_models::ResponseMessage;
use shared::token::JWT;

pub fn update_unit(
    unit_code: &str,
    unit: Json<Unit>,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::units::dsl::{unit_code as db_unit_code, units};

    if token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from("You do not have access to perform this action.")),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

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
                    message: Some(format!("The unit {} could not be found", unit_code)),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
