import styles from './page.module.scss';
import Form from './(components)/(Form)';

export default function Page() {
    return (
        <div className={styles.container}>
            <div className={styles.heading}>
                <h1 className={styles.unitCode}>Submit a review</h1>
            </div>
            <Form />
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
