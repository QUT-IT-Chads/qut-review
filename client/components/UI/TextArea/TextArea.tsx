import { ChangeEventHandler } from 'react';

import FormItem, { FormItemProps } from '@components/UI/FormItem';

interface TextAreaProps extends FormItemProps {
    name: string;
    value: string;
    placeholder: string;
    onChange?: ChangeEventHandler<HTMLTextAreaElement>;
}

export default function TextArea(props: TextAreaProps) {
    return (
        <FormItem id={props.id} label={props.label} required={props.required}>
            <textarea
                id={props.id}
                name={props.name}
                value={props.value}
                placeholder={props.placeholder}
                rows={6}
                onChange={props.onChange}
            />
        </FormItem>
    );
}
