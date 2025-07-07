import { useEffect, useState } from "preact/hooks";
import SideBar from "../components/SideBar.tsx";
import MainContent from "../components/MainContent.tsx";
import { TableRecord } from "../models/summary.ts";

type PatientSummaryProps = {
	patientRecord: TableRecord;
	encountersRecord: Array<TableRecord>;
	inpatientCarePlansRecord: Array<TableRecord>;
	outpatientCarePlansRecord: Array<TableRecord>;
	proceduresRecord: Array<TableRecord>;
};

export default function PatientViewer() {
	const [ids, setIds] = useState<string[]>([]);
	const [patientId, setPatientId] = useState<string | null>(null);
	const [data, setData] = useState<PatientSummaryProps | null>(null);

	useEffect(() => {
		const fetchIds = async () => {
			const res = await fetch("/api/patient_ids");
			const json = await res.json();
			setIds(json);
		};
		fetchIds();
	}, []);

	useEffect(() => {
		if (!patientId) return;
		console.log("Fetching summary for:", patientId);
		const fetchData = async () => {
			const resp = await fetch(`/api/patientsummary/${patientId}`);
			const json = await resp.json();
			setData(json);
		};
		fetchData();
	}, [patientId]);

	return (
		<>
			<SideBar ids={ids} onSelect={setPatientId} />
			{data
				? <MainContent {...data} />
				: (
					<div className="flex items-center justify-center h-full w-full">
						Select a patient to see clinical summary
					</div>
				)}
		</>
	);
}
