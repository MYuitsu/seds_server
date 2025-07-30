import CardLayout from "../components/CardLayout.tsx";
import SelectorButton from "../components/SelectorButton.tsx";
import { selectedEncounter, selectedStudy } from "../signals/patientSummary.ts";
import { formatterDateTime } from "../utils/formatter.ts";

export default function StudySelector() {
    return selectedEncounter.value && (
        <CardLayout>
            <div className="flex overflow-x-auto whitespace-nowrap space-x-4 p-2">
                {selectedEncounter.value.studies.map((study, index) => (
                    <SelectorButton onClick={() => {selectedStudy.value = study;}}
                    selected={selectedStudy.value?.id === study.id}>
                        Study {index + 1}
                    </SelectorButton>
                ))}
            </div>
            {selectedStudy.value
                ? (
                    <>
                        <h2>Imaging Study {selectedStudy.value.id}</h2>
                        <p>Status: {selectedStudy.value.status}</p>
                        <p>Started: {formatterDateTime(selectedStudy.value.started)}</p>
                        <p>Procedure Codes:</p>
                        <ul className="list-disc list-inside">
                            {selectedStudy.value.procedureCode.map((c) => (
                                <li>{c.text}</li>
                            ))}
                        </ul>
                        <p>Location: {selectedStudy.value.location.display}</p>
                    </>
                ) : (
                    <h2>No Study</h2>
                )
            }
        </CardLayout>
    );
}