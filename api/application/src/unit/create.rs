use diesel::prelude::*;
use domain::{models::unit::Unit, enums::role::Role};
use infrastructure::ServerState;
use rocket::{http::Status, response::status::Created, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};

pub fn create_unit(
    unit: Json<Unit>,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::units;

    if token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from("You do not have access to perform this action.")),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    let unit = unit.into_inner();

    let pooled = &mut state.db_pool.get().unwrap();

    let unit_count: i64 = match pooled.transaction(|c| {
        units::table
            .select(units::all_columns)
            .filter(units::unit_code.eq(&unit.unit_code))
            .count()
            .load(c)
    }) {
        Ok(unit_count) => unit_count[0],
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if unit_count > 0 {
        let response = ResponseMessage {
            message: Some(String::from("Unit already exists")),
        };

        return Err((Status::Conflict, Json(response)));
    }

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
