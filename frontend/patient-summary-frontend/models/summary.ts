import { CarePlan } from "./careplan.ts";
import { Encounter } from "./encounter.ts";
import { Patient, renderAddress, renderHumanName } from "./patient.ts";
import {
	Procedure,
	Provenance,
	renderAnnotation,
	renderProcedureBodySite,
} from "./procedure.ts";
import {
	QuestionnaireResponse,
	renderQuestionnaireItem,
} from "./questionnaire.ts";
import {
	renderCodeableConcept,
	renderCoding,
	renderPeriod,
	renderReference,
} from "./shared.ts";

export type PatientSummary = {
	patient: Patient;
	encounters: Encounter[];
	inpatient_careplans: CarePlan[];
	outpatient_careplans: CarePlan[];
	procedures: Array<Procedure | Provenance>;
	questionnaire_responses: QuestionnaireResponse[];
};

export type TableRecord = Record<string, string | string[]>;

export function getPatientRecord({ patient }: PatientSummary): TableRecord {
	return {
		"Patient ID": patient.id,
		"Name": patient.name.map(renderHumanName),
		"Gender": patient.gender || "Unknown",
		"Birth Date": patient.birthDate || "Unknown",
		"Address": patient.address.map(renderAddress),
	};
}

export function getEncountersRecord(
	{ encounters }: PatientSummary,
): Array<TableRecord> {
	return encounters.map((encounter) => ({
		"Encounter ID": encounter.id,
		"Status": encounter.status ?? "Unknown",
		"Class": encounter.class ? renderCoding(encounter.class) : "Unknown",
		"Subject": encounter.subject
			? renderReference(encounter.subject)
			: "Unknown",
		"Period": encounter.period ? renderPeriod(encounter.period) : "Unknown",
		"Diagnosis": encounter.diagnosis
			? encounter.diagnosis.map((d) =>
				d.condition ? renderReference(d.condition) : "Unknown Diagnosis"
			)
			: [],
	}));
}

export function getCarePlansRecord(
	{ inpatient_careplans, outpatient_careplans }: PatientSummary,
): [Array<TableRecord>, Array<TableRecord>] {
	return [
		inpatient_careplans.map((careplan) => ({
			"Care Plan ID": careplan.id,
			"Status": careplan.status ?? "Unknown",
			"Intent": careplan.intent ?? "Unknown",
			"Category": careplan.category
				? renderCodeableConcept(careplan.category)
				: "Unknown",
			"Title": careplan.title ?? "Unknown",
			"Subject": renderReference(careplan.subject),
			"Period": careplan.period
				? renderPeriod(careplan.period)
				: "Unknown",
			"Created": careplan.created ?? "Unknown",
			"Author": careplan.author
				? renderReference(careplan.author)
				: "Unknown",
			"Goals": careplan.goal.map(renderReference),
		})),
		outpatient_careplans.map((careplan) => ({
			"Care Plan ID": careplan.id,
			"Status": careplan.status ?? "Unknown",
			"Intent": careplan.intent ?? "Unknown",
			"Category": careplan.category
				? renderCodeableConcept(careplan.category)
				: "Unknown",
			"Title": careplan.title ?? "Unknown",
			"Subject": renderReference(careplan.subject),
			"Period": careplan.period
				? renderPeriod(careplan.period)
				: "Unknown",
			"Created": careplan.created ?? "Unknown",
			"Author": careplan.author
				? renderReference(careplan.author)
				: "Unknown",
			"Goals": careplan.goal.map(renderReference),
		})),
	];
}

export function getProceduresRecord(
	{ procedures }: PatientSummary,
): Array<TableRecord> {
	return procedures.map((procedure) => {
		switch (procedure.resourceType) {
			case "Procedure":
				return getProcedure(procedure as Procedure);
			case "Provenance":
				return getProvenance(procedure as Provenance);
			default:
				return {};
		}
	});
}

function getProcedure(procedure: Procedure): TableRecord {
	return {
		"resourceType": "Procedure",
		"Procedure ID": procedure.id,
		"Status": procedure.status || "Unknown",
		"Code": procedure.code
			? renderCodeableConcept(procedure.code)
			: "Unknown",
		"Subject": procedure.subject
			? renderReference(procedure.subject)
			: "Unknown",
		"Performed DateTime": procedure.performedDateTime || "Unknown",
		"Body SItes": procedure.body_site
			? procedure.body_site.map(renderProcedureBodySite)
			: [],
		"Notes": procedure.note ? procedure.note.map(renderAnnotation) : [],
	};
}

function getProvenance(provenance: Provenance): TableRecord {
	return {
		"resourceType": "Provenance",
		"Provenance ID": provenance.id,
		"Recorded": provenance.recorded || "Unknown",
		"Target": provenance.target.map(renderReference),
		"Agents": provenance.agent.map((agent) => renderReference(agent.who)),
	};
}

export function getQuestionnaireResponsesRecord(
	{ questionnaire_responses }: PatientSummary,
): Array<TableRecord> {
	return questionnaire_responses.map((response) => ({
		"Response ID": response.id,
		"Status": response.status ?? "Unknown",
		"Subject": renderReference(response.subject),
		"Authored": response.authored ?? "Unknown",
		"Items": response.item
			? response.item.map(renderQuestionnaireItem)
			: [],
	}));
}
