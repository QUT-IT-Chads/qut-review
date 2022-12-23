use chrono::{DateTime, Utc};
use domain::models::{review::Review, unit::Unit, teaching_period::TeachingPeriod};
use domain::enums::semester::Semester;

fn create_dummy_review(unit_code: String, unit_name: String, rating: u8) -> Review {
    Review {
        id: 1, 
        unit: Unit::new_dummy(unit_code, unit_name, String::new()), 
        rating, 
        passed_unit: true, 
        review_body: String::new(),
        teaching_period: TeachingPeriod::new_dummy(
            2022, 
            Semester::Normal(4)
        ),
        date_published: DateTime::<Utc>::default(),    
        last_updated: DateTime::<Utc>::default(),    
        approved: true,
        grade_achieved: None,
    }
}

pub fn get_review() -> Review {
    create_dummy_review(String::from("CAB432"), String::from("Cloud Computing"), 4)
}

pub fn get_reviews() -> Vec<Review> {
    vec![
        create_dummy_review(String::from("CAB432"), String::from("Cloud Computing"), 4),
        create_dummy_review(String::from("IAB230"), String::from("IoT and Mobile Technologies"), 3),
        create_dummy_review(String::from("IFB295"), String::from("Project Management"), 2),
        create_dummy_review(String::from("CAB303"), String::from("Network Security"), 3),
    ]
}
