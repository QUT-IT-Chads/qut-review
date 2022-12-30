use crate::enums::role::Role;
use crate::schema::users;
use diesel::{AsChangeset, Insertable, Queryable};
use rocket::serde::uuid::Uuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Never expose this structure as it contains hashed user password
#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    hashed_password: String,
    pub role: Role,
}

/// Use this structure when returning a user to the frontend
#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct GetUser {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
}

impl From<User> for GetUser {
    fn from(user: User) -> Self {
        GetUser {
            id: user.id,
            email: user.email,
            role: user.role,
        }
    }
}



#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
    pub role: Role,
}

#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct LoginRequest {
    pub email: String,
    pub hashed_password: String,
}

impl From<LoginRequest> for NewUser {
    fn from(user: LoginRequest) -> Self {
        NewUser {
            email: user.email,
            hashed_password: user.hashed_password,
            role: Role::User,
        }
    }
}

impl User {
    pub fn new(id: Uuid, new_user: NewUser) -> Self {
        User {
            id,
            email: new_user.email,
            hashed_password: new_user.hashed_password,
            role: new_user.role,
        }
    }
}
