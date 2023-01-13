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
    id: Uuid,
    email: String,
    hashed_password: String,
    role: Role,
}

impl User {
    pub fn new(id: Uuid, user: UpdateUser) -> Self {
        User {
            id,
            email: user.email,
            hashed_password: user.hashed_password,
            role: user.role,
        }
    }

    /// Returns a version of the User struct which can be publically exposed
    pub fn get_public(&self) -> GetUser {
        GetUser {
            id: self.id,
            email: self.email.clone(),
            role: self.role.clone(),
        }
    }
}

/// A struct to handle a user type which can be publically exposed
#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct GetUser {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
}

/// A struct to handle updating a user
#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: String,
    pub hashed_password: String,
    pub role: Role,
}

/// A struct to handle a users login request
#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct LoginRequest {
    pub email: String,
    pub hashed_password: String,
}

/// A struct to handle the creation of a new user
#[derive(Insertable, Queryable, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
}

impl From<LoginRequest> for UpdateUser {
    fn from(user: LoginRequest) -> Self {
        UpdateUser {
            email: user.email,
            hashed_password: user.hashed_password,
            role: Role::User,
        }
    }
}
