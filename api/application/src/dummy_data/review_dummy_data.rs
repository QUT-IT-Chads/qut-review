use domain::models::*;
use chrono::{DateTime, Utc};

pub fn get_review() -> Review {
    Review::new_dummy(
        1, 
        Unit::new_dummy(String::from("CAB432"), String::from("Cloud Computing"), String::new()), 
        4, 
        true, 
        TeachingPeriod::new_dummy(
            2022, 
            Semester::Standard(2)
        ),
        DateTime::<Utc>::default(),    
        DateTime::<Utc>::default(),    
        true,
        None,
        (),
    )
}
