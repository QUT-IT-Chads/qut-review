use crate::{response_models::ResponseMessage, token::JWT};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use domain::models::user::GetUser;
use infrastructure::{user::read::db_read_user, ServerState};
use rocket::{http::Status, serde::json::Json};
use uuid::Uuid;

use crate::auth::has_user_permissions;

pub fn list_user(
    user_id: Uuid,
    state: &ServerState,
    token: JWT,
) -> Result<GetUser, (Status, Option<String>)> {
    has_user_permissions(&token, &user_id)?;

    db_read_user(&user_id, state)
}

pub fn user_exists(
    user_email: &String,
    pooled: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<bool, (Status, Json<ResponseMessage>)> {
    use domain::schema::users;

    let user_count = pooled
        .transaction(move |c| {
            users::table
                .select(users::all_columns)
                .filter(users::email.eq(&user_email))
                .count()
                .load::<i64>(c)
        })
        .expect("Database error");

    if user_count[0] == 0 {
        return Ok(false);
    }

    Ok(true)
}
