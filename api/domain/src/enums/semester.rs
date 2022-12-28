use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Clone, Deserialize, JsonSchema)]
#[DieselTypePath = "crate::schema::sql_types::Semester"]
pub enum Semester {
    Summer,
    Sem1,
    Sem2,
}
