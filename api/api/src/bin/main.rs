#[macro_use]
extern crate rocket;
use api::{review_handler, unit_handler, user_handler};
use dotenvy::dotenv;
use infrastructure::{establish_connection, ServerState};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let state = ServerState {
        db_pool: establish_connection(),
    };

    rocket::build()
        .manage(state)
        .mount(
            "/api/review",
            routes![
                review_handler::list_reviews_handler,
                review_handler::list_review_handler,
                review_handler::list_user_reviews_handler,
                review_handler::create_review_handler,
                review_handler::approve_review_handler,
                review_handler::delete_review_handler,
                review_handler::update_review_handler,
            ],
        )
        .mount("/api/unit", routes![unit_handler::create_unit_handler])
        .mount("/api/user", routes![user_handler::create_user_handler])
}
