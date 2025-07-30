import { JSX } from "preact/jsx-runtime";

export default function SelectorButton(props: JSX.HTMLAttributes<HTMLButtonElement> & { selected?: boolean }) {
    const base = "p-3 rounded-lg shadow-md border border-gray-200 mb-3";
    return (
        <button 
            type="button" 
            className={props.selected ? `${base} text-white bg-blue-600` : `${base} bg-white`}
            {...props}
        >{props.children}</button>
    );
}
