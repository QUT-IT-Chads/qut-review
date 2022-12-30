use diesel::{prelude::*, r2d2::{PooledConnection, ConnectionManager}};
use domain::{
    enums::role::Role,
    models::user::{GetUser, NewUser, User, LoginRequest},
};
use infrastructure::ServerState;
use rocket::{http::Status, response::status::Created, serde::json::Json, State};
use shared::{response_models::ResponseMessage, token::JWT};
use uuid::Uuid;

pub fn create_user(
    user: Json<LoginRequest>,
    state: &State<ServerState>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {

    let user = user.into_inner();

    let id = Uuid::new_v4();
    let user = User::new(id, user.into());

    let pooled = &mut state.db_pool.get().unwrap();

    insert_user_into_database(user, pooled)
}

pub fn create_admin_user(
    user: Json<NewUser>,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {

    let user = user.into_inner();

    if token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from(
                "You do not have access to perform this action.",
            )),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    let id = Uuid::new_v4();
    let user = User::new(id, user);

    let pooled = &mut state.db_pool.get().unwrap();

    insert_user_into_database(user, pooled)
}

pub fn insert_user_into_database(
    user: User,
    pooled: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Created<String>, (Status, Json<ResponseMessage>)> {
    use domain::schema::users;

    let user_count: i64 = match pooled.transaction(|c| {
        users::table
            .select(users::all_columns)
            .filter(users::email.eq(&user.email))
            .count()
            .load(c)
    }) {
        Ok(user_count) => user_count[0],
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if user_count > 0 {
        let response = ResponseMessage {
            message: Some(String::from("Email is already in use")),
        };

        return Err((Status::Conflict, Json(response)));
    }

    let user: GetUser = match pooled.transaction(|c| {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(c)
    }) {
        Ok(user) => user.into(),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    Ok(Created::new("")
        .tagged_body(serde_json::to_string(&user).expect("Return 500 internal server error.")))
}
