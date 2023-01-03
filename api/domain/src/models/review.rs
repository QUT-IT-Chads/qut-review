use crate::enums::semester::Semester;
use crate::schema::reviews;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Queryable, Serialize, Debug, JsonSchema)]
pub struct Review {
    pub id: i32,
    pub unit_code: String,
    pub rating: i32,
    pub passed_unit: bool,
    pub review_body: String,
    pub teaching_period: Semester,
    pub year_taken: i32,
    pub date_published: NaiveDateTime,
    pub last_updated: NaiveDateTime,
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
            year_taken: 2022,
            date_published: NaiveDateTime::default(),
            last_updated: NaiveDateTime::default(),
            approved: false,
            grade_achieved: new_review.grade_achieved,
            user_id: uuid,
        }
    }
}

#[derive(Insertable, Deserialize, Serialize, Debug, AsChangeset, JsonSchema, Clone)]
#[diesel(table_name = reviews)]
pub struct NewReview {
    pub unit_code: String,
    pub rating: i32,
    pub passed_unit: bool,
    pub review_body: String,
    pub teaching_period: Semester,
    pub year_taken: i32,
    pub grade_achieved: Option<i32>,
}

#[derive(Insertable, Deserialize, Serialize, Debug, AsChangeset, JsonSchema, Clone)]
#[diesel(table_name = reviews)]
pub struct NewReviewWithId {
    pub unit_code: String,
    pub rating: i32,
    pub passed_unit: bool,
    pub review_body: String,
    pub teaching_period: Semester,
    pub year_taken: i32,
    pub grade_achieved: Option<i32>,
    pub user_id: Uuid,
}

impl NewReviewWithId {
    pub fn new(user_id: Uuid, new_review: NewReview) -> Self {
        NewReviewWithId {
            unit_code: new_review.unit_code,
            rating: new_review.rating,
            passed_unit: new_review.passed_unit,
            review_body: new_review.review_body,
            teaching_period: new_review.teaching_period,
            year_taken: new_review.year_taken,
            grade_achieved: new_review.grade_achieved,
            user_id,
        }
    }
}
