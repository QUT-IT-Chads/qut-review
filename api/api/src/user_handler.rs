use application::user::create;
use domain::models::user::NewUser;
use infrastructure::ServerState;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{post, State};

/// Takes in a `NewUser` and returns a 201 Created with the created user as JSON
#[post("/create", format = "application/json", data = "<user>")]
pub fn create_user_handler(user: Json<NewUser>, state: &State<ServerState>) -> Created<String> {
    create::create_user(user, state)
}
