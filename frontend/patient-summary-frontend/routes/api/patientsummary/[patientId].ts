import { Handlers } from "$fresh/server.ts";
import {
	getCarePlansRecord,
	getEncountersRecord,
	getPatientRecord,
	getProceduresRecord,
	getQuestionnaireResponsesRecord,
	PatientSummary,
} from "../../../models/summary.ts";

export const handler: Handlers = {
	async GET(_req, ctx) {
		const { patientId } = ctx.params;
		if (!patientId) {
			return new Response("Patient ID is required", { status: 400 });
		}

		const resp = await fetch(
			`http://0.0.0.0:3000/patient/${patientId}/summary`,
			{
				method: "GET",
				headers: {
					"Content-Type": "application/json",
				},
			},
		);
		const patientSummary: PatientSummary | null = await resp.json();
		if (!patientSummary) {
			return new Response("Patient not found", { status: 404 });
		}

		const patientRecord = getPatientRecord(patientSummary);
		const encountersRecord = getEncountersRecord(patientSummary);
		const [inpatientCarePlansRecord, outpatientCarePlansRecord] =
			getCarePlansRecord(patientSummary);
		const proceduresRecord = getProceduresRecord(patientSummary);
		const questionnaireResponsesRecord = getQuestionnaireResponsesRecord(
			patientSummary,
		);

		return new Response(
			JSON.stringify({
				patientRecord,
				encountersRecord,
				inpatientCarePlansRecord,
				outpatientCarePlansRecord,
				proceduresRecord,
				questionnaireResponsesRecord,
			}),
			{
				headers: {
					"Content-Type": "application/json",
				},
				status: 200,
			},
		);
	},
};
