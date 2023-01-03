// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "semester"))]
    pub struct Semester;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Semester;

    reviews (id) {
        id -> Int4,
        unit_code -> Varchar,
        rating -> Int4,
        passed_unit -> Bool,
        review_body -> Text,
        teaching_period -> Semester,
        year_taken -> Int4,
        date_published -> Timestamp,
        last_updated -> Timestamp,
        approved -> Bool,
        grade_achieved -> Nullable<Int4>,
        user_id -> Uuid,
    }
}

diesel::table! {
    units (unit_code) {
        unit_code -> Varchar,
        unit_name -> Varchar,
        unit_description -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Uuid,
        email -> Varchar,
        hashed_password -> Varchar,
        role -> Role,
    }
}

diesel::joinable!(reviews -> units (unit_code));
diesel::joinable!(reviews -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    reviews,
    units,
    users,
);
