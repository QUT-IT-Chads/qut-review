use crate::schema::users;
use diesel::{Insertable, Queryable};
use rocket::serde::uuid::Uuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize, JsonSchema)]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUserWithUuid {
    id: Uuid,
    email: String,
    hashed_password: String,
}

impl NewUserWithUuid {
    pub fn new(id: Uuid, new_user: NewUser) -> Self {
        NewUserWithUuid {
            id,
            email: new_user.email,
            hashed_password: new_user.hashed_password
        }
    }
}
