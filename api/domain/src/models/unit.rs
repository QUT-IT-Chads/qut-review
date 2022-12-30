use crate::schema::units;
use diesel::{Insertable, Queryable, AsChangeset};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = units)]
pub struct Unit {
    pub unit_code: String,
    pub unit_name: String,
    pub unit_description: String,
}

impl Unit {
    pub fn new(unit_code: String, unit_name: String, unit_description: String) -> Unit {
        Unit {
            unit_code,
            unit_name,
            unit_description,
        }
    }
}
