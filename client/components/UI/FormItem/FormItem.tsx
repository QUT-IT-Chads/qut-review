import { PropsWithChildren } from 'react';
import styles from './FormItem.module.scss';

export interface FormItemProps {
    label: string;
    id: string;
    required: boolean;
}

export default function FormItem(props: PropsWithChildren<FormItemProps>) {
    if (props.required === true) {
        return (
            <div className={styles.container}>
                <label htmlFor={props.id}>
                    {props.label}
                    <span className={styles.required}>*</span>
                </label>
                {props.children}
            </div>
        );
    } else {
        return (
            <div className={styles.container}>
                <label htmlFor={props.id}>{props.label}</label>
                {props.children}
            </div>
        );
    }
}
