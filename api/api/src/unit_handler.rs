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
use shared::response_models::ResponseMessage;
use shared::token::JWT;

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
pub fn list_units_handler(state: &State<ServerState>) -> Json<Vec<Unit>> {
    read::list_units(state)
}

/// Get a unit by unit_code
#[openapi(tag = "Units")]
#[get("/<unit_code>")]
pub fn list_unit_handler(
    unit_code: &str,
    state: &State<ServerState>,
) -> Result<Json<Unit>, (Status, Json<ResponseMessage>)> {
    read::list_unit(unit_code, state)
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

    create::create_unit(unit, state, token)
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

    delete::delete_unit(unit_code, state, token)
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

    update::update_unit(unit_code, unit, state, token)
}
