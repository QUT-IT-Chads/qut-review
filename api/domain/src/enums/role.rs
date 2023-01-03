use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Clone, Deserialize, JsonSchema, Eq, PartialEq, PartialOrd, Ord)]
#[DieselTypePath = "crate::schema::sql_types::Role"]
pub enum Role {
    User,
    Admin,
}
