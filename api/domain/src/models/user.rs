use crate::schema::users;
use diesel::{Insertable, Queryable};
use rocket::serde::uuid::Uuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Debug, Deserialize, Serialize, JsonSchema)]
#[diesel(table_name = users)]
pub struct User {
    id: Uuid,
    email: String,
    hashed_password: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct NewUser {
    email: String,
    hashed_password: String,
}

impl User {
    pub fn new(id: Uuid, new_user: NewUser) -> Self {
        User {
            id,
            email: new_user.email,
            hashed_password: new_user.hashed_password
        }
    }
}
