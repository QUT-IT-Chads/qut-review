import { FormState } from './FormContext';

export enum ActionType {
    SET_UNIT = 'unit',
    SET_TEACHING_PERIOD = 'teachingPeriod',
    SET_YEAR = 'year',
    SET_BODY = 'body',
    SET_GRADE = 'grade',
    SET_PASSED = 'passed',
    SET_RATING = 'rating'
}

export type Action = {
    [K in ActionType]: {
        type: K;
        payload: FormState[K];
    };
}[ActionType];

export default function formReducer(state: FormState, action: Action) {
    switch (action.type) {
        case ActionType.SET_UNIT:
        case ActionType.SET_TEACHING_PERIOD:
        case ActionType.SET_YEAR:
        case ActionType.SET_BODY:
        case ActionType.SET_GRADE:
        case ActionType.SET_PASSED:
        case ActionType.SET_RATING:
            return {
                ...state,
                [action.type]: action.payload
            };
        default:
            throw new Error('Invalid action type');
    }
}
