use crate::dummy_data::review_dummy_data;
use domain::models::review::Review;
use regex::Regex;
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

    NotInDemoMode(String),
    NoDemoData(String),
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DummyResponse {
    pub body: ResponseBody,
}

impl DummyResponse {
    pub fn return_dummy(url: Path) -> Option<ResponseBody> {
        let url_split = url.split('/').skip(1).collect::<Vec<&RawStr>>();

        match url_split[1].as_str() {
            "review" => {
                match url.as_str() {
                    "/api/review/create" | "api/review/update" | "api/review/approve" => {
                        return Some(ResponseBody::Review(review_dummy_data::get_review()));
                    }
                    _ => {
                        let re = Regex::new(r"api/(?:(review/\d+))$").unwrap();
                        let regex_matched = re.is_match(url.as_str());

                        // /api/review/<id>
                        if regex_matched {
                            return Some(ResponseBody::Review(review_dummy_data::get_review()));
                        }

                        let re = Regex::new(r"api/(?:(review/approve/\d+))$").unwrap();
                        let regex_matched = re.is_match(url.as_str());

                        // /api/review/approve/<id>
                        if regex_matched {
                            return Some(ResponseBody::Review(review_dummy_data::get_review()));
                        }

                        // /api/review
                        if url == "/api/review" {
                            return Some(ResponseBody::Reviews(review_dummy_data::get_reviews()));
                        }

                        return None;
                    }
                }
            }
            _ => return None,
        }
    }
}
