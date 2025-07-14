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
	const [advice, setAdvice] = useState<string>("");

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
			const adviceResp = await fetch("/api/advice", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ inpatientCarePlansRecord: json.inpatientCarePlansRecord }),
			});
			const adviceJson = await adviceResp.json();
			setAdvice(adviceJson.advice);
		};
		fetchData();
	}, [patientId]);

	return (
		<div className="flex h-screen">
			<SideBar ids={ids} onSelect={setPatientId} />
			{data
				? (
					<div className="flex flex-col flex-1">
						{/* Main content (2/3 height, scrollable) */}
						<div className="h-2/3 overflow-y-auto">
							<MainContent {...data} />
						</div>
						{/* Advice section (1/3 height) */}
						<div className="h-1/3 p-4 bg-gray-100 overflow-y-auto border-t">
							<h2 className="text-xl font-semibold mb-2">Medical Advice</h2>
							{advice ? (
								<pre className="whitespace-pre-wrap">{advice}</pre>
							) : (
								<p className="text-gray-500">Loading advice...</p>
							)}
						</div>
					</div>
				)
				: (
					<div className="flex items-center justify-center h-full w-full">
						Select a patient to see clinical summary
					</div>
				)}
		</div>
	);
}
