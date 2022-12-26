use application::review::{create, delete, read, update};
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{delete, get, post, put, State};
use shared::response_models::{DummyResponse, NetworkResponse, Response, ResponseBody};

#[get("/")]
pub fn list_reviews_handler(
    demo_mode: Result<DummyResponse, NetworkResponse>,
    state: &State<ServerState>,
) -> String {
    if let Ok(dummy_data) = demo_mode {
        return serde_json::to_string(&dummy_data).expect("Return 500 internal server error.");
    }

    let reviews: Vec<Review> = read::list_reviews(state);

    let response = Response {
        body: ResponseBody::Reviews(reviews),
    };

    serde_json::to_string(&response).expect("Return 500 internal server error.")
}

#[get("/<review_id>")]
pub fn list_review_handler(
    review_id: i32,
    demo_mode: Result<DummyResponse, NetworkResponse>,
    state: &State<ServerState>,
) -> Result<String, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(serde_json::to_string(&dummy_data).expect("Return 500 internal server error."));
    }
    read::list_review(review_id, state)
}

#[get("/user/<user_id>")]
pub fn list_user_reviews_handler(
    user_id: Uuid,
    demo_mode: Result<DummyResponse, NetworkResponse>,
    state: &State<ServerState>,
) -> Result<String, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(serde_json::to_string(&dummy_data).expect("Return 500 internal server error."));
    }
    read::list_user_reviews(user_id, state)
}

#[post("/create", format = "application/json", data = "<review>")]
pub fn create_review_handler(
    review: Json<NewReview>,
    demo_mode: Result<DummyResponse, NetworkResponse>,
    state: &State<ServerState>,
) -> Created<String> {
    if let Ok(dummy_data) = demo_mode {
        return Created::new("").tagged_body(
            serde_json::to_string(&dummy_data).expect("Return 500 internal server error."),
        );
    }

    create::create_review(review, state)
}

#[put("/approve/<review_id>?<status>")]
pub fn approve_review_handler(
    review_id: i32,
    status: Option<bool>,
    demo_mode: Result<DummyResponse, NetworkResponse>,
    state: &State<ServerState>,
) -> Result<String, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(serde_json::to_string(&dummy_data).expect("Return 500 internal server error."));
    }

    update::approve_review(review_id, status.unwrap_or(true), state)
}

#[delete("/<review_id>")]
pub fn delete_review_handler(
    review_id: i32,
    demo_mode: Result<DummyResponse, NetworkResponse>,
    state: &State<ServerState>,
) -> Result<String, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(serde_json::to_string(&dummy_data).expect("Return 500 internal server error."));
    }
    delete::delete_review(review_id, state)
}

#[post("/<review_id>", format = "application/json", data = "<review>")]
pub fn update_review_handler(
    review_id: i32,
    review: Json<NewReview>,
    demo_mode: Result<DummyResponse, NetworkResponse>,
    state: &State<ServerState>,
) -> Result<Created<String>, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(Created::new("").tagged_body(
            serde_json::to_string(&dummy_data).expect("Return 500 internal server error."),
        ));
    }

    update::update_review(review_id, review, state)
}
