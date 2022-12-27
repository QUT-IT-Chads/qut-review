import { ChangeEventHandler } from 'react';

import FormItem from '@components/UI/FormItem';
import styles from './TextArea.module.scss';

interface TextAreaProps {
    label: string;
    id: string;
    name: string;
    placeholder: string;
    required?: boolean;
    eventHandler?: ChangeEventHandler<HTMLTextAreaElement>;
}

export default function TextArea(props: TextAreaProps) {
    if (props.required === undefined) {
        props.required = false;
    }

    return (
        <FormItem id={props.id} label={props.label} required={props.required}>
            <textarea
                className={styles.textArea}
                id={props.id}
                name={props.name}
                placeholder={props.placeholder}
                rows={6}
                onChange={props.eventHandler}
            />
        </FormItem>
    );
}
