'use client';

import { useState } from 'react';

import { NewReview } from 'types/new_review';
import TextArea from '@components/UI/TextArea';

import styles from './Form.module.scss';
import Dropdown from '@components/UI/Dropdown';
import { Option } from 'react-dropdown';

async function sendFormData(newReview: NewReview) {
    await fetch('http://localhost:8000/api/review/create', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(newReview)
    })
        .then((res) => res.json())
        .then((data) => {
            console.log(data);
        });
}

export default function Form() {
    const [reviewBody, setReviewBody] = useState('');

    const gradeOptions: Option[] = [
        { label: '7 - High Distinction', value: '7' },
        { label: '6 - Distinction', value: '6' },
        { label: '5 - Credit', value: '5' },
        { label: '4 - Pass', value: '4' },
        { label: '3 - Marginal Fail', value: '3' },
        { label: '2 - Fail', value: '2' },
        { label: '1 - Low Fail', value: '1' }
    ];

    const defaultGrade = gradeOptions[0];

    return (
        <form className={styles.formGroup}>
            <TextArea
                label="Write a review"
                id="reviewBody"
                name="reviewBody"
                placeholder="Enter your review here..."
                required={true}
                eventHandler={(e) => {
                    setReviewBody(e.target.value);
                }}
            />
            <Dropdown
                label="Grade"
                id="reviewGrade"
                name="reviewGrade"
                options={gradeOptions}
                default={defaultGrade}
                placeholder="Select a grade"
                required={true}
            />
            <button
                type="submit"
                className={styles.submitButton}
                onClick={async (e) => {
                    e.preventDefault();
                    const review: NewReview = {
                        unit_code: 'CAB202',
                        rating: 1,
                        passed_unit: true,
                        review_body: reviewBody,
                        teaching_period: 'Sem1',
                        year_taken: 2022,
                        grade_achieved: 7,
                        user_id: '1'
                    };

                    await sendFormData(review);
                }}
            >
                Submit
            </button>
        </form>
    );
}
