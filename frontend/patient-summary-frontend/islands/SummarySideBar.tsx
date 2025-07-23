import {
	Patient,
	selectedPatient,
	selectedPatientId,
} from "../signals/patientSummary.ts";

export default function SummarySideBar(
	{ patients }: Readonly<{ patients: Pick<Patient, "id" | "names">[] }>,
) {
	const handleFetchSummary = async (patientId: Patient["id"]) => {
		const resp = await fetch(`/demo/patients/${patientId}/summary`, {
			method: "GET",
			headers: {
				"Content-Type": "application/json",
			},
		});
		if (resp.status !== 200) {
			console.log("Error fetching patient's summary by id.");
			return;
		}

		const patient: Patient = await resp.json();
		selectedPatient.value = patient;
        selectedPatientId.value = patientId;
	};

	return (
		<aside className="w-1/5 flex-shrink-0 p-4 bg-gray-100 h-full overflow-y-auto">
			<ul>
				{patients.map((patient) => (
					<li
						key={patient.id}
						id={patient.id}
						className="max-w-4xl w-full p-6 bg-white rounded-lg shadow-md border border-gray-200 mb-6"
					>
						<button
							type="button"
							className="block w-full h-full"
							onClick={() => handleFetchSummary(patient.id)}
						>
							{patient.names[0]?.given?.[0] ||
								patient.names[0]?.family || "Unnamed"}
						</button>
						{selectedPatient.value &&
							selectedPatientId.value === patient.id && (
							<>
								<div>
									{`Gender: ${selectedPatient.value.gender}`}
								</div>
								<div>
									{`DoB: ${selectedPatient.value.birth_date}`}
								</div>
							</>
						)}
					</li>
				))}
			</ul>
		</aside>
	);
}
