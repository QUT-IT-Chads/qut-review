use crate::dummy_data::review_dummy_data;
use domain::models::review::Review;
use rocket::serde::Serialize;
use rocket::{
    http::{uri::Path, RawStr},
    Responder,
};

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 404)]
    NotFound(String),
    DemoMode(String),
}

#[derive(Debug, Serialize)]
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

#[derive(Debug)]
pub struct DummyResponse {
    pub body: ResponseBody,
}

impl DummyResponse {
    pub fn return_dummy(url: Path) -> Option<ResponseBody> {
        let url = url.split('/').skip(1).collect::<Vec<&RawStr>>();

        match url[1].as_str() {
            "review" => {
                // /api/review/
                if url.len() == 2 {
                    return Some(ResponseBody::Reviews(review_dummy_data::get_reviews()));
                }

                // /api/review/<id>
                if url.len() == 3 {
                    return Some(ResponseBody::Review(review_dummy_data::get_review()));
                }
            }
            _ => return None,
        }

        None
    }
}
