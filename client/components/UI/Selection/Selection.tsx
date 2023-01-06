import FormItem, { FormItemProps } from '@components/UI/FormItem';
import Select, {
    ActionMeta,
    GroupBase,
    Props,
    PropsValue,
    SingleValue
} from 'react-select/';

export interface SelectOption {
    label: string;
    value: string;
}

function CustomSelect<
    IsMulti extends boolean = false,
    Group extends GroupBase<SelectOption> = GroupBase<SelectOption>
>(props: Props<SelectOption, IsMulti, Group>) {
    return <Select {...props} />;
}

interface SelectProps extends FormItemProps {
    name: string;
    options: SelectOption[];
    value?: PropsValue<SelectOption>;
    defaultValue?: PropsValue<SelectOption>;
    onChange?: (newValue: SingleValue<SelectOption>, actionMeta: ActionMeta<SelectOption>) => void;
}

export default function Selection(props: SelectProps) {
    return (
        <FormItem id={props.id} label={props.label} required={props.required}>
            <CustomSelect
                id={props.id}
                instanceId={props.id}
                name={props.name}
                value={props.value}
                defaultValue={props.defaultValue}
                onChange={props.onChange}
                options={props.options}
            />
        </FormItem>
    );
}
