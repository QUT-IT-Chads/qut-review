import FormItem, { FormItemProps } from '@components/UI/FormItem';
import Select, {
    GroupBase,
    InputActionMeta,
    Props,
    PropsValue
} from 'react-select';

interface SelectOption {
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
    value: PropsValue<SelectOption>;
    options: SelectOption[];
    onChange?: (newValue: string, actionMeta: InputActionMeta) => void;
}

export default function Selection(props: SelectProps) {
    return (
        <FormItem id={props.id} label={props.label} required={props.required}>
            <CustomSelect
                id={props.id}
                name={props.name}
                value={props.value}
                onInputChange={props.onChange}
                options={props.options}
            />
        </FormItem>
    );
}
