use chrono::{DateTime, Utc};
use domain::enums::semester::Semester;
use domain::models::{review::Review, unit::Unit};
use uuid::Uuid;

fn create_dummy_review(unit_code: String, unit_name: String, rating: u8) -> Review {
    let uuid = Uuid::new_v4();

    Review {
        id: 1,
        unit: Unit::new(unit_code, unit_name, String::new()),
        rating,
        passed_unit: true,
        review_body: String::new(),
        teaching_period: Semester::Sem1,
        date_published: DateTime::<Utc>::default(),
        last_updated: DateTime::<Utc>::default(),
        approved: true,
        grade_achieved: None,
        user_id: uuid,
    }
}

pub fn get_review() -> Review {
    create_dummy_review(String::from("CAB432"), String::from("Cloud Computing"), 4)
}

pub fn get_reviews() -> Vec<Review> {
    vec![
        create_dummy_review(String::from("CAB432"), String::from("Cloud Computing"), 4),
        create_dummy_review(
            String::from("IAB230"),
            String::from("IoT and Mobile Technologies"),
            3,
        ),
        create_dummy_review(
            String::from("IFB295"),
            String::from("Project Management"),
            2,
        ),
        create_dummy_review(String::from("CAB303"), String::from("Network Security"), 3),
    ]
}
