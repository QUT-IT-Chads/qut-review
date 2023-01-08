use application::response_models::ResponseMessage;
use application::token::JWT;
use application::unit::{create, delete, read, update};
use domain::models::unit::Unit;
use infrastructure::ServerState;
use okapi::openapi3::OpenApi;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post, State};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: create_unit_handler,
        list_units_handler,
        list_unit_handler,
        delete_unit_handler,
        update_unit_handler,
    ]
}

/// Get a list of all units
#[openapi(tag = "Units")]
#[get("/")]
pub fn list_units_handler(
    state: &State<ServerState>,
) -> Result<Json<Vec<Unit>>, (Status, Json<ResponseMessage>)> {
    let state = state.inner();

    match read::list_units(state) {
        Ok(units) => Ok(Json(units)),
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}

/// Get a unit by unit_code
#[openapi(tag = "Units")]
#[get("/<unit_code>")]
pub fn list_unit_handler(
    unit_code: &str,
    state: &State<ServerState>,
) -> Result<Json<Unit>, (Status, Json<ResponseMessage>)> {
    let state = state.inner();

    match read::list_unit(unit_code, state) {
        Ok(unit) => Ok(Json(unit)),
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}

/// Create a new unit
#[openapi(tag = "Units")]
#[post("/create", format = "application/json", data = "<unit>")]
pub fn create_unit_handler(
    unit: Json<Unit>,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    let token = token?;
    let unit = unit.into_inner();
    let state = state.inner();

    match create::create_unit(unit, state, token) {
        Ok(unit) => Ok(Created::new("")
            .tagged_body(serde_json::to_string(&unit).expect("Return 500 internal server error."))),
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}

/// Delete a unit
#[openapi(tag = "Units")]
#[delete("/<unit_code>")]
pub fn delete_unit_handler(
    unit_code: &str,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Json<ResponseMessage>, (Status, Json<ResponseMessage>)> {
    let token = token?;
    let state = state.inner();

    match delete::delete_unit(unit_code, state, token) {
        Ok(message) => {
            let response = ResponseMessage { message };
            Ok(Json(response))
        }
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}

/// Update a unit
#[openapi(tag = "Units")]
#[post("/<unit_code>", format = "application/json", data = "<unit>")]
pub fn update_unit_handler(
    unit_code: &str,
    unit: Json<Unit>,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    let token = token?;
    let state = state.inner();
    let unit = unit.into_inner();

    match update::update_unit(unit_code, unit, state, token) {
        Ok(unit) => Ok(Created::new("")
            .tagged_body(serde_json::to_string(&unit).expect("Return 500 internal server error."))),
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}
