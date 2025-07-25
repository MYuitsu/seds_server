import { useSignalEffect } from "@preact/signals";
import { selectedEncounter, selectedPatient, selectedObservation, selectedCondition, selectedStudy, selectedDiagnosis, groupedObservations } from "../signals/patientSummary.ts";
import Diagram from "./Diagram.tsx";
import MapNavigator from "../components/MapNavigator.tsx";
import { JSX } from "preact/jsx-runtime";

export default function SummaryMainContent() {
    useSignalEffect(() => {
        selectedEncounter.value = selectedPatient.value ? selectedPatient.value.encounters[0] : null;
        selectedObservation.value = selectedPatient.value?.encounters[0].observations ? selectedPatient.value.encounters[0].observations[0] : null;
        selectedCondition.value = selectedPatient.value?.encounters[0].conditions ? selectedPatient.value.encounters[0].conditions[0] : null;
        selectedStudy.value = selectedPatient.value?.encounters[0].studies ? selectedPatient.value.encounters[0].studies[0] : null;
        selectedDiagnosis.value = selectedPatient.value?.encounters[0].diagnosis ? selectedPatient.value.encounters[0].diagnosis[0] : null;
    })

	return selectedPatient.value
		? (
			<div className="flex flex-col h-full overflow-hidden">
				{selectedPatient.value.encounters.length > 0
					? generateMapNavigator() 
					: <h2>No Encounter</h2>
				}
			</div>
		)
		: (
			<div className="flex flex-col items-center justify-center h-full w-full overflow-hidden">
				Press on patients to see their clinical summary.
			</div>
		);
}

function generateMapNavigator(): JSX.Element | null {
	const grouped = groupedObservations.value;
	if (!grouped) return null;

	const outerMap: Record<string, JSX.Element> = {};

	for (const category in grouped) {
		const codeMap: Record<string, JSX.Element> = {};

		for (const code in grouped[category]) {
			const observations = grouped[category][code];
			const hasValueQuantity = observations.some(
				(obs) => obs.valueQuantity?.value !== undefined,
			);

			if (hasValueQuantity) {
				const label = code.includes(" in ") ? code.split(" in ")[0] : code;
				codeMap[label] = <Diagram category={category} code={code} />;
			}
		}

		if (Object.keys(codeMap).length > 0) {
			outerMap[category] = <MapNavigator map={codeMap} />;
		}
	}

	return <MapNavigator map={outerMap} />;
}
