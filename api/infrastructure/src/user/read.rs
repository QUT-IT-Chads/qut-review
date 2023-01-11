use crate::ServerState;
use diesel::prelude::*;
use domain::models::user::{GetUser, User};
use rocket::http::Status;
use uuid::Uuid;

pub fn db_read_user(
    user_id: &Uuid,
    state: &ServerState,
) -> Result<GetUser, (Status, Option<String>)> {
    use domain::schema::users;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| users::table.find(user_id).first::<User>(c)) {
        Ok(user) => Ok(user.get_public()),
        Err(diesel::result::Error::NotFound) => Err((
            Status::NotFound,
            Some(String::from("User could not be found.")),
        )),
        Err(err) => panic!("Database error - {}", err),
    }
}

pub fn db_login_request(
    user_email: &String,
    user_hashed_password: &String,
    state: &ServerState,
) -> Result<GetUser, (Status, Option<String>)> {
    use domain::schema::users;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        users::table
            .select(users::all_columns)
            .filter(users::email.eq(&user_email))
            .filter(users::hashed_password.eq(&user_hashed_password))
            .first::<User>(c)
    }) {
        Ok(user) => Ok(user.get_public()),
        Err(diesel::result::Error::NotFound) => Err((
            Status::Unauthorized,
            Some(String::from("Invalid credentials")),
        )),
        Err(err) => panic!("Database error - {}", err),
    }
}

pub fn db_does_user_exist(
    user_email: &String,
    state: &ServerState,
) -> Result<bool, (Status, Option<String>)> {
    use domain::schema::users;

    let pooled = &mut state.db_pool.get().unwrap();

    let user_count = pooled
        .transaction(move |c| {
            users::table
                .select(users::all_columns)
                .filter(users::email.eq(&user_email))
                .count()
                .load::<i64>(c)
        })
        .expect("Database error");

    Ok(user_count[0] != 0)
}
