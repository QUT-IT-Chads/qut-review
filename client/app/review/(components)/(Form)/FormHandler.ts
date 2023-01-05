import { NewReview } from 'types/new_review';

export default async function formHandler(newReview: NewReview) {
    await fetch(`${process.env.API_URL}/review/create`, {
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

// const enum GRADE {
//     HIGH_DISTINCTION = 7,
//     DISTINCTION = 6,
//     CREDIT = 5,
//     PASS = 4,
//     MARGINAL_FAIL = 3,
//     FAIL = 2,
//     LOW_FAIL = 1
// }

// const GRADE_CUTOFFS = {
//     [GRADE.HIGH_DISTINCTION]: 85,
//     [GRADE.DISTINCTION]: 75,
//     [GRADE.CREDIT]: 65,
//     [GRADE.PASS]: 50,
//     [GRADE.MARGINAL_FAIL]: 40,
//     [GRADE.FAIL]: 30,
//     [GRADE.LOW_FAIL]: 0
// };
