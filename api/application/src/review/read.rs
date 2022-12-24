use diesel::prelude::*;
use domain::models::review::Review;
use infrastructure::ServerState;
use rocket::{response::status::NotFound, State};
use shared::response_models::{Response, ResponseBody};

pub fn list_review(review_id: u32) -> Result<Review, NotFound<String>> {
    let response = Response {
        body: ResponseBody::Message(format!("Error: review with ID {} not found", review_id)),
    };

    Err(NotFound(
        serde_json::to_string(&response).expect("Return 500 internal server error."),
    ))
}

pub fn list_reviews(state: &State<ServerState>) -> Vec<Review> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| reviews::table.load::<Review>(c)) {
        Ok(reviews) => reviews,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
