use crate::enums::grade::Grade;
use crate::models::{teaching_period::TeachingPeriod, unit::Unit};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Review {
    pub id: u32,
    pub unit: Unit,
    pub rating: u8,
    pub passed_unit: bool,
    pub review_body: String,
    pub teaching_period: TeachingPeriod,
    pub date_published: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    /// Approval by admin - Review should not be visible until approved
    pub approved: bool,
    pub grade_achieved: Option<Grade>,
    // Todo
    pub user: (),
}

impl Review {
    /// !Warning: This should only be called when creating dummy data.
    pub fn new(new_review: NewReview) -> Review {
        Review {
            id: 1,
            unit: new_review.unit,
            rating: new_review.rating,
            review_body: new_review.review_body,
            passed_unit: new_review.passed_unit,
            teaching_period: new_review.teaching_period,
            date_published: DateTime::<Utc>::default(),
            last_updated: DateTime::<Utc>::default(),
            approved: false,
            grade_achieved: new_review.grade_achieved,
            user: (),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewReview {
    unit: Unit,
    rating: u8,
    passed_unit: bool,
    review_body: String,
    teaching_period: TeachingPeriod,
    grade_achieved: Option<Grade>,
}
