use diesel::prelude::*;
use domain::models::unit::Unit;
use infrastructure::ServerState;
use rocket::{http::Status, response::status::Created, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};

use crate::auth::has_admin_permissions;
use crate::database_helpers::unit_exists;

pub fn create_unit(
    unit: Json<Unit>,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::units;

    if let Err(err) = has_admin_permissions(&token) {
        return Err(err);
    }

    let unit = unit.into_inner();

    let pooled = &mut state.db_pool.get().unwrap();

    match unit_exists(&unit.unit_code, pooled) {
        Ok(exists) => {
            if exists {
                let response = ResponseMessage {
                    message: Some(String::from("Unit already exist.")),
                };

                return Err((Status::Conflict, Json(response)));
            }
        }
        Err(err) => return Err(err),
    };

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

    Ok(Created::new("")
        .tagged_body(serde_json::to_string(&unit).expect("Return 500 internal server error.")))
}
