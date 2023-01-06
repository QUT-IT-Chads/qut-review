import styles from './page.module.scss';
import Form from './(components)/(Form)';
import { Unit } from 'types/unit';

async function getData() {
    try {
        const response = await fetch(
            `${process.env.API_URL}/api/unit`,
            {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json'
                }
            }
        );
        return (await response.json());
    }
    catch (error) {
        // console.log(error);
    }
}

export default async function Page() {
    const data: Unit[] = await getData();

    const unitOptions = data.map((u: Unit) => {
        return {
            label: `${u.unit_code} - ${u.unit_name}`,
            value: u.unit_code
        };
    });
    // const unitOptions = [
    //     {
    //         label: 'cab201-programming principles',
    //         value: 'cab201'
    //     },
    //     {
    //         label: 'cab202-microprocessors',
    //         value: 'cab202'
    //     }
    // ]

    return (
        <div className={styles.container}>
            <div className={styles.heading}>
                <h1>Submit a review</h1>
            </div>
            <Form unitOptions={unitOptions} />
            <div className={styles.info}>
                <div>
                    Only your review, rating, and pass status will be displayed
                    to other users. If enough data is present, your grade will
                    be used to generate a grade distribution.
                </div>
            </div>
        </div>
    );
}
