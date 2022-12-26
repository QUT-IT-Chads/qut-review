use application::unit::create;
use domain::models::unit::Unit;
use infrastructure::ServerState;
use okapi::openapi3::OpenApi;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_unit_handler]
}

/// Create a new unit
#[openapi(tag = "Units")]
#[post("/create", format = "application/json", data = "<unit>")]
pub fn create_unit_handler(unit: Json<Unit>, state: &State<ServerState>) -> Created<String> {
    create::create_unit(unit, state)
}
