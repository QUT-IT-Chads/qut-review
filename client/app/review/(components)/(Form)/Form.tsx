'use client';
import { useSearchParams } from 'next/navigation';

import { useEffect, useState } from 'react';
import styles from './Form.module.scss';

import Numeric from '@components/UI/Numeric';
import Selection, { SelectOption } from '@components/UI/Selection';
import Checkbox from '@components/UI/Checkbox';
import TextArea from '@components/UI/TextArea';

import formHandler from './FormHandler';
import FormState from './FormState';
import { Semester } from 'types/semester';
import { Validator } from '@helper/Validator';

interface FormProps {
    unitOptions: SelectOption[];
}

export default function Form(props: FormProps) {
    const [review, setReview] = useState<FormState>({
        unit: '',
        semester: 'Sem1',
        year: new Date().getFullYear(),
        body: '',
        grade: 100,
        passed: true,
        rating: 5
    });

    const semesters = [
        {
            label: 'Semester 1',
            value: 'Sem1' as Semester
        },
        {
            label: 'Semester 2',
            value: 'Sem2' as Semester
        },
        {
            label: 'Summer',
            value: 'Summer' as Semester
        }
    ];

    const searchParams = useSearchParams();

    useEffect(() => {
        if (searchParams.has('code')) {
            const code = searchParams.get('code');
            if (code != null) {
                if (props.unitOptions.find((u) => u.value.toLowerCase() === code.toLowerCase())) {
                    setReview({ ...review, unit: code });
                }
            }
        }
    }, []);

    return (
        <form
            className={styles.group}
            onSubmit={(e) => formHandler(e, review)}
            id="review"
        >
            <Selection
                label="Select a unit"
                id="reviewUnit"
                name="reviewUnit"
                options={props.unitOptions}
                value={props.unitOptions.find(
                    (u) =>
                        u.value.toLowerCase() === review.unit.toLowerCase()
                ) as SelectOption}
                required={true}
                onChange={(newValue, actionMeta) => {
                    if (actionMeta.action === 'select-option') {
                        if (newValue !== null) {
                            if (newValue.value !== null) {
                                setReview({
                                    ...review,
                                    unit: newValue.value
                                });
                            }
                        }
                    }
                }}
            />
            <div className={styles.split}>
                <Selection
                    label="Semester"
                    id="reviewSemester"
                    name="reviewSemester"
                    options={semesters}
                    defaultValue={semesters[0]}
                    required={true}
                    onChange={(newValue, actionMeta) => {
                        if (actionMeta.action === 'select-option') {
                            if (newValue !== null) {
                                if (newValue.value !== null) {
                                    setReview({
                                        ...review,
                                        semester: newValue.value as Semester
                                    });
                                }
                            }
                        }
                    }}
                />
                <Numeric
                    label="Year"
                    id="reviewYear"
                    name="reviewYear"
                    placeholder={new Date().getFullYear()}
                    value={review.year}
                    min={2000}
                    max={new Date().getFullYear()}
                    step={1}
                    required={true}
                    onChange={(e) => {
                        if (e.target.value === '') {
                            setReview({ ...review, year: 2000 });
                        } else if (
                            Validator.isValidNumber(
                                parseInt(e.target.value),
                                2000,
                                new Date().getFullYear(),
                                1
                            )
                        ) {
                            setReview({
                                ...review,
                                year: parseInt(e.target.value)
                            });
                        } else {
                            console.log('Invalid year', e.target.value);
                        }
                    }}
                />
            </div>
            <TextArea
                label="Write a review"
                id="reviewBody"
                name="reviewBody"
                value={review.body}
                placeholder="Enter your review here..."
                required={true}
                onChange={(e) => setReview({ ...review, body: e.target.value })}
            />
            <Numeric
                label="Grade"
                id="reviewGrade"
                name="reviewGrade"
                placeholder={100}
                value={review.grade}
                min={0}
                max={100}
                step={1}
                required={false}
                onChange={(e) => {
                    if (e.target.value === '') {
                        setReview({ ...review, grade: 100 });
                    } else if (
                        Validator.isValidNumber(
                            parseInt(e.target.value),
                            0,
                            100,
                            1
                        )
                    ) {
                        setReview({
                            ...review,
                            grade: parseInt(e.target.value)
                        });
                    } else {
                        console.log('Invalid grade', e.target.value);
                    }
                }}
            />
            <Checkbox
                label="Did you pass this unit?"
                id="reviewPassQ"
                name="reviewPassQ"
                value={review.passed}
                required={true}
                onChange={(e) =>
                    setReview({ ...review, passed: e.target.checked })
                }
            />
            <button
                id="reviewSubmit"
                type="submit"
                className={styles.submitButton}
                form="review"
            >
                Submit
            </button>
        </form>
    );
}
