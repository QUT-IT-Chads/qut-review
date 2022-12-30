import styles from './Header.module.scss';

export default function Header() {
    return (
        <header className={styles.container}>
            <div className={styles.containerLeft}>
                <h1>QUT Review</h1>
                <h2>Search</h2>
            </div>
            <div className={styles.containerRight}>
                {/* TODO: change to next/link  */}
                <button className={styles.reviewWrite}>Write a Review</button>
            </div>
        </header>
    );
}
