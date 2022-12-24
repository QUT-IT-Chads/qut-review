use crate::enums::semester::Semester;
use crate::schema::reviews;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Queryable, Serialize, Debug)]
pub struct Review {
    pub id: i32,
    pub unit_code: String,
    pub rating: i32,
    pub passed_unit: bool,
    pub review_body: String,
    pub teaching_period: Semester,
    pub date_published: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    /// Approval by admin - Review should not be visible until approved
    pub approved: bool,
    pub grade_achieved: Option<i32>,
    pub user_id: Uuid,
}

impl Review {
    /// !Warning: This should only be called when creating dummy data.
    pub fn new(new_review: NewReview) -> Review {
        let uuid = Uuid::new_v4();

        Review {
            id: 1,
            unit_code: new_review.unit_code,
            rating: new_review.rating,
            review_body: new_review.review_body,
            passed_unit: new_review.passed_unit,
            teaching_period: new_review.teaching_period,
            date_published: NaiveDateTime::default(),
            last_updated: NaiveDateTime::default(),
            approved: false,
            grade_achieved: new_review.grade_achieved,
            user_id: uuid,
        }
    }
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = reviews)]
pub struct NewReview {
    unit_code: String,
    rating: i32,
    passed_unit: bool,
    review_body: String,
    teaching_period: Semester,
    grade_achieved: Option<i32>,
    user_id: Uuid,
}
