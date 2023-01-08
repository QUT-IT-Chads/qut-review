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
        Ok(user) => {
            return Ok(user.get_public());
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err((
                    Status::NotFound,
                    Some(String::from("User could not be found.")),
                ));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
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
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err((
                    Status::Unauthorized,
                    Some(String::from("Invalid credentials")),
                ));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn db_does_user_exist(
    user_email: &String,
    state: &ServerState,
) -> Result<bool, (Status, Option<String>)> {
    use domain::schema::users;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| {
        users::table
            .select(users::all_columns)
            .filter(users::email.eq(&user_email))
            .count()
            .load::<i64>(c)
    }) {
        Ok(user_count) => {
            if user_count[0] == 0 {
                return Ok(false);
            }

            Ok(true)
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
