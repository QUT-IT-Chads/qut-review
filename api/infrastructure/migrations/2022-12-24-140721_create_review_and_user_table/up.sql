-- Your SQL goes here
CREATE TYPE Semester AS ENUM (
    'summer',
    'sem1',
    'sem2'
);

CREATE TYPE Role AS ENUM (
    'user',
    'admin'
);

CREATE TABLE Units (
    unit_code VARCHAR PRIMARY KEY,
    unit_name VARCHAR NOT NULL NOT NULL,
    unit_description TEXT NOT NULL
);

CREATE TABLE Users (
    id UUID PRIMARY KEY,
    email VARCHAR NOT NULL,
    hashed_password VARCHAR NOT NULL,
    role Role DEFAULT 'user' NOT NULL
);

CREATE TABLE Reviews (
    id SERIAL PRIMARY KEY,
    unit_code VARCHAR NOT NULL,
    CONSTRAINT fk_unit_code
        FOREIGN KEY(unit_code)
            REFERENCES Units(unit_code),
    rating INT NOT NULL,
    passed_unit BOOLEAN NOT NULL,
    review_body TEXT NOT NULL,
    teaching_period Semester NOT NULL,
    year_taken INT NOT NULL,
    date_published timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_updated timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    approved BOOLEAN DEFAULT FALSE NOT NULL,
    grade_achieved INT,
    user_id UUID NOT NULL,
    CONSTRAINT fk_author
        FOREIGN KEY(user_id) 
            REFERENCES Users(id)
                ON DELETE CASCADE
);
