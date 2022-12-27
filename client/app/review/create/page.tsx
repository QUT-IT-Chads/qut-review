import styles from './page.module.scss';
import Form from './(components)/(Form)';

export default function Page() {
    return (
        <div className={styles.formContainer}>
            <div className={styles.formHeading}>
                <h1 className={styles.unitCode}>CAB202</h1>
                <h2 className={styles.unitName}>
                    Microprocessors and Digital Systems
                </h2>
            </div>
            <Form />
        </div>
    );
}
