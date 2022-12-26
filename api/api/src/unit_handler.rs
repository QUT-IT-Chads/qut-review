use application::unit::create;
use domain::models::unit::Unit;
use infrastructure::ServerState;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{post, State};

/// Takes in a `Unit` and returns a 201 Created with the created unit as JSON
#[post("/create", format = "application/json", data = "<unit>")]
pub fn create_unit_handler(unit: Json<Unit>, state: &State<ServerState>) -> Created<String> {
    create::create_unit(unit, state)
}
