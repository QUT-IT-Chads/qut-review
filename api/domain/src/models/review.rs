use crate::enums::grade::Grade;
use crate::models::{teaching_period::TeachingPeriod, unit::Unit};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Review {
    id: u32,
    unit: Unit,
    rating: u8,
    passed_unit: bool,
    review_body: String,
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
    pub fn new_dummy(
        id: u32,
        unit: Unit,
        rating: u8,
        review_body: String,
        passed_unit: bool,
        teaching_period: TeachingPeriod,
        date_published: DateTime<Utc>,
        last_updated: DateTime<Utc>,
        approved: bool,
        grade_achieved: Option<Grade>,
        user: (),
    ) -> Review {
        Review {
            id,
            unit,
            rating,
            review_body,
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
