use rocket::Responder;
use domain::models::Review;
use rocket::serde::Serialize;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 404)]
    NotFound(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),

    Review(Review),
    Reviews(Vec<Review>),

    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
