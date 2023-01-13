use application::response_models::ResponseMessage;
use application::review::{create, delete, read, update};
use application::token::JWT;
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use okapi::openapi3::OpenApi;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{delete, get, post, put, State};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use crate::{convert_err, convert_result};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: list_review_handler,
        list_reviews_handler,
        list_unit_reviews_handler,
        list_user_reviews_handler,
        create_review_handler,
        approve_review_handler,
        delete_review_handler,
        update_review_handler,
    ]
}

/// Get a list of all reviews
#[openapi(tag = "Reviews")]
#[get("/?<_page>&<_limit>")]
pub fn list_reviews_handler(
    _page: Option<i64>,
    _limit: Option<i64>,
    state: &State<ServerState>,
) -> Result<Json<Vec<Review>>, (Status, Json<ResponseMessage>)> {
    let state = state.inner();

    convert_result(read::list_reviews(_page, _limit, state))
}

/// Get a list of all reviews sorted by unit code
#[openapi(tag = "Reviews")]
#[get("/unit/<unit_code>?<_page>&<_limit>")]
pub fn list_unit_reviews_handler(
    unit_code: String,
    _page: Option<i64>,
    _limit: Option<i64>,
    state: &State<ServerState>,
) -> Result<Json<Vec<Review>>, (Status, Json<ResponseMessage>)> {
    let state = state.inner();

    convert_result(read::list_unit_reviews(unit_code, _page, _limit, state))
}

/// Get a review by ID
#[openapi(tag = "Reviews")]
#[get("/<review_id>")]
pub fn list_review_handler(
    review_id: i32,
    state: &State<ServerState>,
) -> Result<Json<Review>, (Status, Json<ResponseMessage>)> {
    let state = state.inner();

    convert_result(read::list_review(review_id, state))
}

/// Get all reviews by a user
#[openapi(tag = "Reviews")]
#[get("/user/<user_id>")]
pub fn list_user_reviews_handler(
    user_id: Uuid,
    state: &State<ServerState>,
) -> Result<Json<Vec<Review>>, (Status, Json<ResponseMessage>)> {
    let state = state.inner();

    convert_result(read::list_user_reviews(user_id, state))
}

/// Create a new review
#[openapi(tag = "Reviews")]
#[post("/create", format = "application/json", data = "<review>")]
pub fn create_review_handler(
    review: Json<NewReview>,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    let token = token?;
    let state = state.inner();

    create::create_review(review.into_inner(), state, token)
        .map(|review| {
            Created::new("").tagged_body(
                serde_json::to_string(&review).expect("Return 500 internal server error."),
            )
        })
        .map_err(convert_err)
}

/// Approve or disapprove a review
#[openapi(tag = "Reviews")]
#[put("/approve/<review_id>?<status>")]
pub fn approve_review_handler(
    review_id: i32,
    status: Option<bool>,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Json<Review>, (Status, Json<ResponseMessage>)> {
    let token = token?;
    let state = state.inner();

    convert_result(update::approve_review(
        review_id,
        status.unwrap_or(true),
        state,
        token,
    ))
}

/// Delete a review
#[openapi(tag = "Reviews")]
#[delete("/<review_id>")]
pub fn delete_review_handler(
    review_id: i32,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Json<ResponseMessage>, (Status, Json<ResponseMessage>)> {
    let token = token?;
    let state = state.inner();

    delete::delete_review(review_id, state, token)
        .map(|message| Json(ResponseMessage { message }))
        .map_err(convert_err)
}

/// Update a review
#[openapi(tag = "Reviews")]
#[post("/<review_id>", format = "application/json", data = "<review>")]
pub fn update_review_handler(
    review_id: i32,
    review: Json<NewReview>,
    state: &State<ServerState>,
    token: Result<JWT, (Status, Json<ResponseMessage>)>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    let token = token?;
    let state = state.inner();

    let review = review.into_inner();

    update::update_review(review_id, review, state, token)
        .map(|review| {
            Created::new("")
                .tagged_body(serde_json::to_string(&review).expect("500 internal server error"))
        })
        .map_err(convert_err)
}
