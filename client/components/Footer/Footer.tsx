import styles from './Footer.module.scss';

export default function Footer() {
    return (
        <footer className={styles.container}>
            <div className="pb-2">
                <a href="#" className={styles.item}>Privacy</a>
                <a href="#" className={styles.item}>Terms</a>
                <a href="#" className={styles.item}>Github</a>
            </div>
            <span className={styles.text}>QUT Review is not affiliated with, nor endorsed by the Queensland University of Technology. © IT Chads 2023</span>
        </footer>
    );
}
