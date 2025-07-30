import { TableRecord } from "../models/summary.ts";
import CardLayout from "./CardLayout.tsx";
import TableData from "./TableRecord.tsx";

type PatientSummaryProps = {
	patientRecord: TableRecord;
	encountersRecord: Array<TableRecord>;
	inpatientCarePlansRecord: Array<TableRecord>;
	outpatientCarePlansRecord: Array<TableRecord>;
	proceduresRecord: Array<TableRecord>;
};

export default function PatientSummaryById(
	data: Readonly<PatientSummaryProps>,
) {
	return (
		<div className="max-w-6xl w-full mx-auto p-6 grid grid-cols-2 gap-6 h-screen overflow-y-auto">
			<CardLayout>
				<TableData
					title="Patient Information"
					data={data.patientRecord}
				/>
			</CardLayout>
			{data.encountersRecord.map((record, index) => (
				<CardLayout key={record["Encounter ID"]}>
					<TableData
						title={`Encounter ${index + 1}`}
						data={record}
					/>
				</CardLayout>
			))}
			{data.inpatientCarePlansRecord.map((record, index) => (
				<CardLayout key={record["Care Plan ID"]}>
					<TableData
						title={`Inpatient Care Plan ${index + 1}`}
						data={record}
					/>
				</CardLayout>
			))}
			{data.outpatientCarePlansRecord.map((record, index) => (
				<CardLayout key={record["Care Plan ID"]}>
					<TableData
						title={`Outpatient Care Plan ${index + 1}`}
						data={record}
					/>
				</CardLayout>
			))}
			{data.proceduresRecord.map((record, index) => (
				(
					<CardLayout key={record["Procedure ID"]}>
						<TableData
							title={`Procedure ${index + 1}`}
							data={record}
						/>
					</CardLayout>
				)
			))}
		</div>
	);
}
