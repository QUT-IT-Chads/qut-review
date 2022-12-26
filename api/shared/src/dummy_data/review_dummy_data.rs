use chrono::NaiveDateTime;
use domain::enums::semester::Semester;
use domain::models::review::Review;
use uuid::Uuid;

fn create_dummy_review(unit_code: String, rating: i32) -> Review {
    let uuid = Uuid::new_v4();

    Review {
        id: 1,
        unit_code,
        rating,
        passed_unit: true,
        review_body: String::new(),
        teaching_period: Semester::Sem1,
        date_published: NaiveDateTime::default(),
        last_updated: NaiveDateTime::default(),
        approved: true,
        grade_achieved: None,
        user_id: uuid,
    }
}

pub fn get_review() -> Review {
    create_dummy_review(String::from("CAB432"), 4)
}

pub fn get_reviews() -> Vec<Review> {
    vec![
        create_dummy_review(String::from("CAB432"), 4),
        create_dummy_review(String::from("IAB230"), 3),
        create_dummy_review(String::from("IFB295"), 2),
        create_dummy_review(String::from("CAB303"), 3),
    ]
}
