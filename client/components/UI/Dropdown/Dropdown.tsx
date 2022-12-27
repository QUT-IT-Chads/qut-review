import FormItem from '../FormItem';
// import styles from './Dropdown.module.scss';
import {default as ReactDropdown, Group, Option} from 'react-dropdown';

interface DropdownProps {
    label: string;
    id: string;
    name: string;
    options: (Group | Option | string)[];
    default: Option | string;
    placeholder: string;
    required: boolean;
}

export default function Dropdown(props: DropdownProps) {
    return (
        <FormItem id={props.id} label={props.label} required={props.required}>
            <ReactDropdown
                options={props.options}
                value={props.default}
                placeholder={props.placeholder}
            />
            {/* <select className={styles.dropdownSelect} id={props.id}>
                {props.options.map((option, index) => {
                    if (index === props.default) {
                        (
                            <option value={option} selected>
                                {option}
                            </option>
                        );
                    } else {
                        return <option value={option}>{option}</option>;
                    }
                }}
                </select> */}
        </FormItem>
    );
}
