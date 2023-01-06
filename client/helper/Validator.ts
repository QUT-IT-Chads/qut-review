import FormState from '@app/review/(components)/(Form)/FormState';
import { Semester } from 'types/semester';
import { Unit } from 'types/unit';

const semesterKeys = ['Summer', 'Sem1', 'Sem2'];

export abstract class Validator {
    public static isValidNumber(
        value: number,
        min: number,
        max: number,
        step: number
    ): boolean {
        return (min <= value) && (value <= max) && ((value - min) % step === 0);
    }

    public static isValidYear(value: number): boolean {
        return this.isValidNumber(value, 2000, new Date().getFullYear(), 1);
    }
}

export abstract class ReviewValidator extends Validator {
    public static isUnitValid(unit: string, units: Unit[]): boolean {
        console.log(units);
        return units.some((u) => u.unit_code === unit);
    }

    public static isSemesterValid(semester: string): semester is Semester {
        return semesterKeys.includes(semester);
    }

    public static isReviewValid(review: FormState, units: Unit[]): boolean {
        if (review.grade === null) {
            return this.isUnitValid(review.unit, units);
        } else {
            return (
                this.isUnitValid(review.unit, units) &&
                this.isValidNumber(review.grade, 0, 100, 1)
            );
        }
    }
}
