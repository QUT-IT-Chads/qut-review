pub mod unit;
pub mod review;
pub mod user;

use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct ServerState {
    pub db_pool: DbPool,
}

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.")
}
