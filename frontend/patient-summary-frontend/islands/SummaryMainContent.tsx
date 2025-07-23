import { useSignalEffect } from "@preact/signals";
import { selectedEncounter, selectedPatient, selectedObservation, selectedCondition, selectedStudy, selectedDiagnosis } from "../signals/patientSummary.ts";
import SelectorButton from "../components/SelectorButton.tsx";
import ObservationSelector from "./ObservationSelector.tsx";
import ConditionSelector from "./ConditionSelector.tsx";
import StudySelector from "./StudySelector.tsx";
import DiagnosisSelector from "./DiagnosisSelector.tsx";
import { formatterPeriod } from "../utils/formatter.ts";

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
					? (
						<>
							<div className="h-1/4 max-w-6xl p-4 bg-blue-100">
								<div className="flex overflow-x-auto whitespace-nowrap space-x-4 p-2">
									{selectedPatient.value.encounters.map((
										encounter,
										index,
									) => (
										<SelectorButton
											onClick={() => {
												try {
													console.log(
														"Clicked encounter:",
														encounter,
													);
													selectedEncounter.value =
														encounter;
													selectedObservation.value =
														encounter.observations
															?.[0] ?? null;
													selectedCondition.value =
														encounter.conditions
															?.[0] ?? null;
													selectedStudy.value =
														encounter.studies
															?.[0] ?? null;
													selectedDiagnosis.value =
														encounter.diagnosis
															?.[0] ?? null;
													console.log(
														"Selected encounter now:",
														selectedEncounter.value
															?.id,
													);
												} catch (e) {
													console.error(
														"Failed to update encounter state:",
														e,
													);
												}
											}}
											selected={selectedEncounter.value
												?.id === encounter.id}
										>
											{`Encounter ${index + 1}`}
										</SelectorButton>
									))}
								</div>
								{selectedEncounter.value
									? (
										<>
                                            <h2>{`Encounter ${selectedEncounter.value.id}`}</h2>
                                            <p>Status: {selectedEncounter.value.status}</p>
                                            <p>{formatterPeriod(selectedEncounter.value.period.start, selectedEncounter.value.period.end)}</p>
                                        </>
									)
									: <h2>No Encounter Selected</h2>}
							</div>
							<div className="h-3/4 p-4 bg-blue-200 grid grid-cols-2 gap-3 overflow-y-auto">
								<ObservationSelector />
								<ConditionSelector />
								<StudySelector />
								<DiagnosisSelector />
							</div>
						</>
					)
					: <h2>No Encounter</h2>}
			</div>
		)
		: (
			<div className="flex flex-col items-center justify-center h-full w-full overflow-hidden">
				Press on patients to see their clinical summary.
			</div>
		);
}