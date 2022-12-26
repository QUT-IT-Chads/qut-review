use application::user::create;
use domain::models::user::NewUser;
use infrastructure::ServerState;
use okapi::openapi3::OpenApi;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_user_handler]
}

/// Create a new user
#[openapi(tag = "Users")]
#[post("/create", format = "application/json", data = "<user>")]
pub fn create_user_handler(user: Json<NewUser>, state: &State<ServerState>) -> Created<String> {
    create::create_user(user, state)
}
