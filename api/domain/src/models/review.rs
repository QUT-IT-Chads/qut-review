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
    pub user_id: Uuid,
}
