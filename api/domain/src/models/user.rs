use crate::schema::users;
use diesel::{Insertable, Queryable, AsChangeset};
use rocket::serde::uuid::Uuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Never expose this structure as it contains hashed user password
#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    id: Uuid,
    pub email: String,
    hashed_password: String,
}

/// Use this structure when returning a user to the frontend
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetUser {
    id: Uuid,
    email: String,
}

impl From<User> for GetUser {
    fn from(user: User) -> Self {
        GetUser {
            id: user.id,
            email: user.email,
        }
    }
}

#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
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
