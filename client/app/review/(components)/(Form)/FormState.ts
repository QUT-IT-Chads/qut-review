import { Semester } from 'types/new_review';
export default interface FormState {
    unit: string;
    semester: Semester;
    year: number;
    body: string;
    grade: number;
    passed: boolean;
    rating: number;
}
