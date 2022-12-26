use crate::schema::units;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = units)]
pub struct Unit {
    unit_code: String,
    unit_name: String,
    unit_description: String,
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
