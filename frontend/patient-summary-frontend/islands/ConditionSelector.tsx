import CardLayout from "../components/CardLayout.tsx";
import SelectorButton from "../components/SelectorButton.tsx";
import { selectedEncounter, selectedCondition } from "../signals/patientSummary.ts";
import { formatterDateTime } from "../utils/formatter.ts";

export default function ConditionSelector() {
    return selectedEncounter.value && (
        <CardLayout>
            <div className="flex overflow-x-auto whitespace-nowrap space-x-4 p-2">
                {selectedEncounter.value.conditions.map((condition, index) => (
                    <SelectorButton onClick={() => {selectedCondition.value = condition;}}
                    selected={selectedCondition.value?.id === condition.id}>
                        Condition {index + 1}
                    </SelectorButton>
                ))}
            </div>
            {selectedCondition.value
                ? (
                    <>
                        <h2>Condition {selectedCondition.value.id}</h2>
                        <p>Status: {selectedCondition.value.clinicalStatus.coding.map((stat) => stat.code).join(', ')} - {selectedCondition.value.verificationStatus.coding.map((stat) => stat.code).join(', ')}</p>
                        <p>Categories: {selectedCondition.value.category.map((c) => c.coding.map((d) => d.display).join(', ')).join(', ')}</p>
                        <p>Code: {selectedCondition.value.code.text}</p>
                        <p>On Set Date Time: {formatterDateTime(selectedCondition.value.onsetDateTime)}</p>
                        <p>Recorded Date: {formatterDateTime(selectedCondition.value.recordedDate)}</p>
                    </>
                ) : (
                    <h2>No Condition</h2>
                )
            }
        </CardLayout>
    );
}