use application::user::{create, update, read, delete};
use domain::models::user::{NewUser, User};
use infrastructure::ServerState;
use okapi::openapi3::OpenApi;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{delete, get, post, State};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use shared::response_models::ResponseMessage;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: create_user_handler,
        list_user_handler,
        delete_user_handler,
        update_user_handler
    ]
}

/// Create a new user
#[openapi(tag = "Users")]
#[post("/create", format = "application/json", data = "<user>")]
pub fn create_user_handler(user: Json<NewUser>, state: &State<ServerState>) -> Created<String> {
    create::create_user(user, state)
}

/// Get a user by id
#[openapi(tag = "Users")]
#[get("/<user_id>")]
pub fn list_user_handler(
    user_id: Uuid,
    state: &State<ServerState>,
) -> Result<Json<User>, NotFound<Json<ResponseMessage>>> {
    read::list_user(user_id, state)
}

/// Delete a user
#[openapi(tag = "Users")]
#[delete("/<user_id>")]
pub fn delete_user_handler(
    user_id: Uuid,
    state: &State<ServerState>,
) -> Result<Json<ResponseMessage>, NotFound<Json<ResponseMessage>>> {
    delete::delete_user(user_id, state)
}

/// Update a user
#[openapi(tag = "Users")]
#[post("/<user_id>", format = "application/json", data = "<user>")]
pub fn update_user_handler(
    user_id: Uuid,
    user: Json<User>,
    state: &State<ServerState>,
) -> Result<Created<String>, NotFound<Json<ResponseMessage>>> {
    update::update_user(user_id, user, state)
}
