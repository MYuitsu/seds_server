import CardLayout from "../components/CardLayout.tsx";
import SelectorButton from "../components/SelectorButton.tsx";
import { selectedEncounter, selectedDiagnosis } from "../signals/patientSummary.ts";
import { formatterDateTime } from "../utils/formatter.ts";

export default function DiagnosisSelector() {
    return selectedEncounter.value && (
        <CardLayout>
            <div className="flex overflow-x-auto whitespace-nowrap space-x-4 p-2">
                {selectedEncounter.value.diagnosis.map((diagnosis, index) => (
                    <SelectorButton onClick={() => {selectedDiagnosis.value = diagnosis;}}
                    selected={selectedDiagnosis.value?.id === diagnosis.id}>
                        Diagnosis {index + 1}
                    </SelectorButton>
                ))}
            </div>
            {selectedDiagnosis.value
                ? (
                    <>
                        <h2>Diagnosis Report {selectedDiagnosis.value.id}</h2>
                        <p>Status: {selectedDiagnosis.value.status}</p>
                        <p>Effective Date Time: {formatterDateTime(selectedDiagnosis.value.effectiveDateTime)}</p>
                        <p>Issued: {formatterDateTime(selectedDiagnosis.value.issued)}</p>
                        <p>Performers:</p>
                        <ul className="list-disc list-inside">
                            {selectedDiagnosis.value.performer.map((p) => (
                                <li>{p.display}</li>
                            ))}
                        </ul>
                    </>
                ) : (
                    <h2>No Diagnosis</h2>
                )
            }
        </CardLayout>
    );
}