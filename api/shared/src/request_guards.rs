use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use std::env;

use crate::response_models::{DummyResponse, NetworkResponse};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DummyResponse {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        let demo_mode: bool = env::var("DEMO")
            .expect("DEMO env variable required.")
            .parse()
            .unwrap_or_default();

        if demo_mode {
            let response = match DummyResponse::return_dummy(req.uri().path()) {
                Some(response_data) => response_data,
                None => {
                    return Outcome::Failure((
                        Status::NotFound,
                        NetworkResponse::NoDemoData(String::from("not demo mode")),
                    ))
                }
            };

            return Outcome::Success(DummyResponse { body: response });
        }

        Outcome::Failure((
            Status::Processing,
            NetworkResponse::NotInDemoMode(String::from("not demo mode")),
        ))
    }
}
