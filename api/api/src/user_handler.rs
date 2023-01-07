use application::response_models::{AuthToken, ResponseMessage};
use application::token::JWT;
use application::user::{create, delete, login, read, update};
use domain::models::user::{GetUser, LoginRequest, UpdateUser};
use infrastructure::ServerState;
use okapi::openapi3::OpenApi;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{delete, get, post, State};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: create_user_handler,
        list_user_handler,
        delete_user_handler,
        update_user_handler,
        login_user_handler,
    ]
}

/// Create a new user
#[openapi(tag = "Users")]
#[post("/create", format = "application/json", data = "<user>")]
pub fn create_user_handler(
    user: Json<LoginRequest>,
    state: &State<ServerState>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    let user = user.into_inner();

    match create::create_user(user, state) {
        Ok(user) => Ok(Created::new("")
            .tagged_body(serde_json::to_string(&user).expect("Return 500 internal server error."))),
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}

/// Get a user by id
#[openapi(tag = "Users")]
#[get("/<user_id>")]
pub fn list_user_handler(
    user_id: Uuid,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Json<GetUser>, (Status, Json<ResponseMessage>)> {
    let token = token?;

    match read::list_user(user_id, state, token) {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}

/// Delete a user
#[openapi(tag = "Users")]
#[delete("/<user_id>")]
pub fn delete_user_handler(
    user_id: Uuid,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Json<ResponseMessage>, (Status, Json<ResponseMessage>)> {
    let token = token?;

    match delete::delete_user(user_id, state, token) {
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

/// Update a user
#[openapi(tag = "Users")]
#[post("/<user_id>", format = "application/json", data = "<user>")]
pub fn update_user_handler(
    user_id: Uuid,
    user: Json<UpdateUser>,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    let token = token?;

    let user = user.into_inner();

    match update::update_user(user_id, user, state, token) {
        Ok(user) => Ok(Created::new("")
            .tagged_body(serde_json::to_string(&user).expect("Return 500 internal server error."))),
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}

/// Login as a user
#[openapi(tag = "Users")]
#[post("/login", format = "application/json", data = "<user>")]
pub fn login_user_handler(
    user: Json<LoginRequest>,
    state: &State<ServerState>,
) -> Result<Json<AuthToken>, (Status, Json<ResponseMessage>)> {
    let user = user.into_inner();

    match login::login_user(user, state) {
        Ok(token) => {
            let response = AuthToken { token };
            Ok(Json(response))
        }
        Err(err) => {
            let response = ResponseMessage { message: err.1 };
            Err((err.0, Json(response)))
        }
    }
}
