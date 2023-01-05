import { createContext, Dispatch } from 'react';
import { Semester } from 'types/new_review';
import { Action } from './FormReducer';

export interface FormState {
    unit: string;
    teachingPeriod: Semester;
    year: number;
    body: string;
    grade: number;
    passed: boolean;
    rating: number;
}

type T = { state: FormState; dispatch: Dispatch<Action> };

export const FormContext = createContext<T>({} as T);
