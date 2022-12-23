use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Unit {
    unit_code: String,
    unit_name: String,
    unit_description: String,
}

impl Unit {
    pub fn new_dummy(unit_code: String, unit_name: String, unit_description: String) -> Unit {
        Unit {
            unit_code,
            unit_name,
            unit_description,
        }
    }
}
