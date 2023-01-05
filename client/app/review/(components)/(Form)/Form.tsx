'use client';
import { useReducer } from 'react';
// import { useSearchParams } from 'next/navigation';

import styles from './Form.module.scss';

import Numeric from '@components/UI/Numeric';
import Select from '@components/UI/Selection';
import Checkbox from '@components/UI/Checkbox';
import TextArea from '@components/UI/TextArea';

import { NewReview } from 'types/new_review';
import { Unit } from 'types/unit';

import { ActionType } from './FormReducer';
import { FormContext, FormState } from './FormContext';
import formHandler from './FormHandler';

// import { ReviewValidator } from '@helper/Validator';
import formReducer from './FormReducer';

export default async function Form() {
    const initialState = {
        unit: '',
        teachingPeriod: 'Sem1',
        year: new Date().getFullYear(),
        body: '',
        grade: 100,
        passed: true,
        rating: 5
    } as FormState;

    const [state, dispatch] = useReducer(formReducer, initialState);

    // const searchParams = useSearchParams();
    // useEffect(() => {
    //     if (searchParams.has('code')) {
    //         const code = searchParams.get('code');
    //         if (code != null) {
    //             if (ReviewValidator.isUnitValid(code, units)) {
    //                 dispatch({
    //                     type: ActionType.SET_UNIT,
    //                     payload: code
    //                 });
    //             }
    //         }
    //     }
    // }, []);

    // useEffect(() => {
    //     async function getUnits() {
    //         const res = await fetch(`${process.env.API_URL}/unit`);
    //         setUnits(await res.json());
    //     }

    //     getUnits();
    // }, []);

    // if (error) return <div>failed to load</div>;
    // if (isLoading) return <div>loading...</div>;

    // if (units === undefined) return <div>data error...</div>;

    const units = [] as Unit[];

    return (
        <FormContext.Provider value={{ state, dispatch }}>
            <form className={styles.group}>
                <Select
                    label="Select a unit"
                    id="reviewUnit"
                    name="reviewUnit"
                    options={units.map((unit) => {
                        return {
                            label: `${unit.unit_code} - ${unit.unit_name}`,
                            value: unit.unit_code
                        };
                    })}
                    value={units
                        .filter((unit) => unit.unit_code === state.unit)
                        .map((unit) => {
                            return {
                                label: `${unit.unit_code} - ${unit.unit_name}`,
                                value: unit.unit_code
                            };
                        })}
                    required={true}
                    onChange={(newValue) => {
                        dispatch({
                            type: ActionType.SET_UNIT,
                            payload: newValue
                        });
                    }}
                />
                <TextArea
                    label="Write a review"
                    id="reviewBody"
                    name="reviewBody"
                    placeholder="Enter your review here..."
                    required={true}
                    onChange={(e) => {
                        dispatch({
                            type: ActionType.SET_BODY,
                            payload: e.target.value
                        });
                    }}
                />
                <Numeric
                    label="Grade"
                    id="reviewGrade"
                    name="reviewGrade"
                    placeholder={100}
                    min={0}
                    max={100}
                    step={1}
                    required={false}
                    onChange={(e) => {
                        dispatch({
                            type: ActionType.SET_GRADE,
                            payload: parseInt(e.target.value)
                        });
                    }}
                />
                <Checkbox
                    label="Did you pass this unit?"
                    id="reviewPassQ"
                    name="reviewPassQ"
                    value={state.passed}
                    required={true}
                    onChange={(e) => {
                        dispatch({
                            type: ActionType.SET_PASSED,
                            payload: e.target.checked
                        });
                    }}
                />
                <input
                    id="reviewSubmit"
                    type="submit"
                    className={styles.submitButton}
                    onClick={async (e) => {
                        e.preventDefault();

                        const review: NewReview = {
                            unit_code: state.unit,
                            review_body: state.body,
                            teaching_period: state.teachingPeriod,
                            year_taken: state.year,
                            grade_achieved: state.grade,
                            passed_unit: state.passed,
                            rating: state.rating,
                            user_id: '1'
                        };

                        await formHandler(review);
                    }}
                    value="Submit"
                />
            </form>
        </FormContext.Provider>
    );
}
