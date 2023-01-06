import { FormEvent } from 'react';
import { NewReview } from 'types/new_review';
import FormState from './FormState';

function ReviewConvert(state: FormState): NewReview {
    return {
        unit_code: state.unit,
        teaching_period: state.semester,
        year_taken: state.year,
        review_body: state.body,
        grade_achieved: state.grade,
        passed_unit: state.passed,
        rating: state.rating
    } as NewReview;
}

export default async function formHandler(e: FormEvent<HTMLFormElement>, state: FormState) {
    e.preventDefault();

    await fetch(`${process.env.API_URL}/api/review/create`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer <token>'
        },
        body: JSON.stringify(ReviewConvert(state))
    })
        .catch((err) => {
            console.log(err);
        });
}
