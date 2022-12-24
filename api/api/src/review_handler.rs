use application::review::{create, delete, read, update};
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use shared::response_models::{DummyResponse, NetworkResponse, Response, ResponseBody};

/// Returns a 200 OK containing JSON vector of all reviews
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

/// Takes in a `review_id` and returns a 200 OK with the associated review as JSON
/// otherwise, a 404 NotFound NetworkResponse.
#[get("/<review_id>")]
pub fn list_review_handler(
    review_id: u32,
    demo_mode: Result<DummyResponse, NetworkResponse>,
) -> Result<String, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(serde_json::to_string(&dummy_data).expect("Return 500 internal server error."));
    }
    let review: Review = read::list_review(review_id)?;

    let response = Response {
        body: ResponseBody::Review(review),
    };

    Ok(serde_json::to_string(&response).expect("Return 500 internal server error."))
}

/// Takes in a `NewReview` and returns a 201 Created with the created review as JSON
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

/// Takes in a `review_id` and an approved `status` bool returning a 201 Created with the
/// associated review as JSON otherwise, a 404 NotFound NetworkResponse.
///
/// If no `status` is provided, it will default to `true`
#[put("/approve/<review_id>?<status>")]
pub fn approve_review_handler(
    review_id: u32,
    status: Option<bool>,
    demo_mode: Result<DummyResponse, NetworkResponse>,
) -> Result<Created<String>, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(Created::new("").tagged_body(
            serde_json::to_string(&dummy_data).expect("Return 500 internal server error."),
        ));
    }

    update::approve_review(review_id, status.unwrap_or(true))
}

/// Takes in a `review_id` and returns a 200 OK
/// otherwise, a 404 NotFound NetworkResponse
#[delete("/<review_id>")]
pub fn delete_review_handler(
    review_id: u32,
    demo_mode: Result<DummyResponse, NetworkResponse>,
) -> Result<String, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(serde_json::to_string(&dummy_data).expect("Return 500 internal server error."));
    }
    delete::delete_review(review_id)
}

/// Takes in a `review_id` and `NewReview` object returning a 201 Created with the updated review as JSON
/// otherwise, a 404 NotFound NetworkResponse
#[post("/<review_id>", format = "application/json", data = "<review>")]
pub fn update_review_handler(
    review_id: u32,
    review: Json<NewReview>,
    demo_mode: Result<DummyResponse, NetworkResponse>,
) -> Result<Created<String>, NotFound<String>> {
    if let Ok(dummy_data) = demo_mode {
        return Ok(Created::new("").tagged_body(
            serde_json::to_string(&dummy_data).expect("Return 500 internal server error."),
        ));
    }

    update::update_review(review_id, review)
}
