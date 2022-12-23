use domain::models::{review::Review, unit::Unit, teaching_period::TeachingPeriod};
use domain::enums::semester::Semester;
use chrono::{DateTime, Utc};

fn create_dummy_review(id: u32, unit_code: String, unit_name: String, rating: u8) -> Review {
    Review::new_dummy(
        id, 
        Unit::new_dummy(unit_code, unit_name, String::new()), 
        rating, 
        String::new(),
        true, 
        TeachingPeriod::new_dummy(
            2022, 
            Semester::Normal(4)
        ),
        DateTime::<Utc>::default(),    
        DateTime::<Utc>::default(),    
        true,
        None,
        (),
    )
}

pub fn get_review() -> Review {
    create_dummy_review(1, String::from("CAB432"), String::from("Cloud Computing"), 4)
}

pub fn get_reviews() -> Vec<Review> {
    vec![
        create_dummy_review(1, String::from("CAB432"), String::from("Cloud Computing"), 4),
        create_dummy_review(2, String::from("IAB230"), String::from("IoT and Mobile Technologies"), 3),
        create_dummy_review(3, String::from("IFB295"), String::from("Project Management"), 2),
        create_dummy_review(4, String::from("CAB303"), String::from("Network Security"), 3),
    ]
}
