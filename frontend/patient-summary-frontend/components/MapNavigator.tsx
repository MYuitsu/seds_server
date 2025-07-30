import { useSignal } from "@preact/signals";
import { JSX } from "preact/jsx-runtime";

type NavMap = {
    map: Record<string, JSX.Element>,
}

export default function MapNavigator({map}: Readonly<NavMap>) {
    const tabNames = Object.keys(map);
    const activeTab = useSignal(tabNames[0]);

    const ActiveComponent = map[activeTab.value];

    return (
        <div className="flex flex-col space-y-4">
            <div className="flex overflow-x-auto whitespace-nowrap space-x-2 pb-2">
                {tabNames.map((name) => (
                    <button
                        key={name}
                        className={
                            `px-4 py-2 rounded ${name === activeTab.value 
                                ? "bg-blue-500 text-white" 
                                : "bg-gray-200"}`
                        }
                        onClick={() => {activeTab.value = name;}}
                        type="button"
                    >
                        {name}
                    </button>
                ))}
            </div>
            <div className="p-4 min-w-0">
                {ActiveComponent || <p>No tab selected.</p>}
            </div>
        </div>
    );
}