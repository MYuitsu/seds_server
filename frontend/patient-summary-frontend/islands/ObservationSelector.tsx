import CardLayout from "../components/CardLayout.tsx";
import SelectorButton from "../components/SelectorButton.tsx";
import { Observation, selectedEncounter, selectedObservation, groupedObservations } from "../signals/patientSummary.ts";
import { formatterDateTime } from "../utils/formatter.ts";

export default function ObservationSelector() {
    console.log(groupedObservations.value);
    return selectedEncounter.value && (
        <CardLayout>
            <div className="flex overflow-x-auto whitespace-nowrap space-x-4 p-2">
                {selectedEncounter.value.observations.map((observation, index) => (
                    <SelectorButton onClick={() => {selectedObservation.value = observation;}}
                    selected={selectedObservation.value?.id === observation.id}>
                        Observation {index + 1}
                    </SelectorButton>
                ))}
            </div>
            {selectedObservation.value
                ? (
                    <>
                        <h2>Observation {selectedObservation.value.id}</h2>
                        <p>Status: {selectedObservation.value.status}</p>
                        <p>Categories: {selectedObservation.value.category.map((c) => c.coding.map((d) => d.display).join(', ')).join(', ')}</p>
                        <p>Code: {selectedObservation.value.code.text}</p>
                        <p>Issued: {formatterDateTime(selectedObservation.value.issued)}</p>
                        <p>Quantity: {renderQuantity(selectedObservation.value)}</p>
                    </>
                ) : (
                    <h2>No Observation</h2>
                )
            }
        </CardLayout>
    );
}

function renderQuantity(observation: Observation): string {
    if (observation.component.length > 0) {
        return observation.component.map(
            (c) => `${c.code.text} ${c.valueQuantity 
                ? `${c.valueQuantity.value} (${c.valueQuantity.unit})` 
                : ''}`)
            .join(', ')
    } else {
        return observation.valueQuantity
            ? `${observation.valueQuantity.value} (${observation.valueQuantity.unit})` 
            : '';
    }
}
