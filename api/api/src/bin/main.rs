#[macro_use] extern crate rocket;
use api::review_handler;
use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/api/review", routes![
            review_handler::list_reviews_handler, 
            review_handler::list_review_handler,
            review_handler::create_review_handler,
        ])
}
