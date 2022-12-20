#[macro_use] extern crate rocket;
use api::review_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/review", routes![
            review_handler::list_reviews_handler, 
            review_handler::list_review_handler,
        ])
}
