use rocket::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Unit {
    unit_code: String,
    unit_name: String,
    unit_description: String,
}

impl Unit {
    pub fn new_dummy(unit_code: String, unit_name: String, unit_description: String) -> Unit {
        Unit {
            unit_code, unit_name, unit_description
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeachingPeriod {
    year: u32,
    semester: Semester,
}

impl TeachingPeriod {
    pub fn new_dummy(year: u32, semester: Semester) -> TeachingPeriod {
        TeachingPeriod {
            year, semester
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Semester {
    Normal(u8),
    Summer,
    Other,
}

/// Users achieved grade
#[derive(Debug, Deserialize, Serialize)]
pub enum Grade {
    /// Grade: 1-100%
    Percentage(u8),
    /// Grade: 1-7
    Scale(u8),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Review {
    id: u32,
    unit: Unit,
    rating: u8,
    passed_unit: bool,
    teaching_period: TeachingPeriod,
    date_published: DateTime<Utc>,
    last_updated: DateTime<Utc>,
    /// Approval by admin - Review should not be visible until approved
    approved: bool,
    grade_achieved: Option<Grade>,
    // Todo
    user: (),
}

impl Review {
    pub fn new_dummy(id: u32, unit: Unit, rating: u8, passed_unit: bool, teaching_period: TeachingPeriod, date_published: DateTime<Utc>, last_updated: DateTime<Utc>, approved: bool, grade_achieved: Option<Grade>, user: ()) -> Review {
        Review {
            id,
            unit,
            rating,
            passed_unit,
            teaching_period,
            date_published,
            last_updated,
            approved,
            grade_achieved,
            user,
        }
    }
}
