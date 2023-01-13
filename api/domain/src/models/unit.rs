use crate::schema::units;
use diesel::{AsChangeset, Insertable, Queryable};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize, JsonSchema, AsChangeset)]
#[diesel(table_name = units)]
pub struct Unit {
    pub unit_code: String,
    pub unit_name: String,
    pub unit_description: String,
}
