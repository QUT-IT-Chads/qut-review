use application::unit::{create, read, delete, update};
use domain::models::unit::Unit;
use infrastructure::ServerState;
use okapi::openapi3::OpenApi;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post, delete, State};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use shared::response_models::ResponseMessage;

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
) -> Result<Json<Unit>, NotFound<Json<ResponseMessage>>> {
    read::list_unit(unit_code, state)
}

/// Create a new unit
#[openapi(tag = "Units")]
#[post("/create", format = "application/json", data = "<unit>")]
pub fn create_unit_handler(unit: Json<Unit>, state: &State<ServerState>) -> Created<String> {
    create::create_unit(unit, state)
}

/// Delete a unit
#[openapi(tag = "Units")]
#[delete("/<unit_code>")]
pub fn delete_unit_handler(
    unit_code: &str,
    state: &State<ServerState>,
) -> Result<Json<ResponseMessage>, NotFound<Json<ResponseMessage>>> {
    delete::delete_unit(unit_code, state)
}

/// Update a unit
#[openapi(tag = "Units")]
#[post("/<unit_code>", format = "application/json", data = "<unit>")]
pub fn update_unit_handler(
    unit_code: &str,
    unit: Json<Unit>,
    state: &State<ServerState>,
) -> Result<Created<String>, NotFound<Json<ResponseMessage>>> {
    update::update_unit(unit_code, unit, state)
}
