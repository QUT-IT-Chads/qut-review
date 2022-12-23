use crate::enums::semester::Semester;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeachingPeriod {
    year: u32,
    semester: Semester,
}

impl TeachingPeriod {
    pub fn new_dummy(year: u32, semester: Semester) -> TeachingPeriod {
        TeachingPeriod { year, semester }
    }
}
