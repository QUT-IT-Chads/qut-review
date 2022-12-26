use serde::{Deserialize, Serialize};

/// Users achieved grade
#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Deserialize)]
#[DieselTypePath = "crate::schema::sql_types::Semester"]
pub enum Semester {
    Summer,
    Sem1,
    Sem2,
}
