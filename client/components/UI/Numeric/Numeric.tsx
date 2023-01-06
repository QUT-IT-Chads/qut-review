import { ChangeEventHandler } from 'react';

import FormItem, { FormItemProps } from '@components/UI/FormItem';

interface NumericProps extends FormItemProps {
    name: string;
    placeholder: number;
    value: number;
    onChange?: ChangeEventHandler<HTMLInputElement>;
    min: number;
    max: number;
    step: number;
}

export default function Numeric(props: NumericProps) {
    return (
        <FormItem id={props.id} label={props.label} required={props.required}>
            <input
                type="number"
                id={props.id}
                name={props.name}
                placeholder={props.placeholder.toString()}
                value={props.value}
                onChange={props.onChange}
                min={props.min}
                max={props.max}
                step={props.step}
            />
        </FormItem>
    );
}
