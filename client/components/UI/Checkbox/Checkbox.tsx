import { ChangeEventHandler } from 'react';

import FormItem, { FormItemProps } from '@components/UI/FormItem';

interface CheckboxProps extends FormItemProps {
    name: string;
    value: boolean;
    onChange?: ChangeEventHandler<HTMLInputElement>;
}

export default function Checkbox(props: CheckboxProps) {
    return (
        <FormItem id={props.id} label={props.label} required={props.required}>
            <input
                type="checkbox"
                id={props.id}
                name={props.name}
                checked={props.value}
                onChange={props.onChange}
            />
        </FormItem>
    );
}
