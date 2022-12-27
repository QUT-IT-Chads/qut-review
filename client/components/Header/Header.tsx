import styles from './Header.module.scss';

export default function Header() {
    return (
        <header className={styles.container}>
            <div className="flex items-end">
                <h1 className={styles.title}><a href="#">QUT Review</a></h1>
                <div className="ml-3">
                    <a href="#" className={styles.item}>Search</a>
                    <a href="#"className={styles.item}>Review</a>
                </div>
            </div>
            <div className="flex items-end">
                <a href="#"className={styles.item}>Register</a>
                <a href="#"className={styles.item}>Login</a>
            </div>
        </header>
    );
}
