import { CarePlan } from "./careplan.ts";
import { Encounter, renderParticipant } from "./encounter.ts";
import { Patient, renderAddress, renderCommunication, renderHumanName, renderPractitioner } from "./patient.ts";
import { Procedure } from "./procedure.ts";
import { renderCodeableConcept, renderPeriod, renderReference } from "./shared.ts";

export type PatientSummary = {
	patient: Patient;
	encounters: Encounter[];
	inpatient_careplans: CarePlan[];
	outpatient_careplans: CarePlan[];
	procedures: Procedure[];
};

export type TableRecord = Record<string, string | string[]>;

export function getPatientRecord({ patient }: PatientSummary): TableRecord {
	return {
		"ID": patient.id["@value"] ?? "_",
		"Name": patient.name.map(renderHumanName),
		"Gender": patient.gender ? patient.gender["@value"] ?? "_" : "_",
		"Date of Birth": patient.birthDate ? patient.birthDate["@value"] ?? "_" : "_",
		"Address": patient.address.map(renderAddress),
		"Marital Status": patient.maritalStatus.text["@value"] ?? "_",
		"Communications": patient.communication.map(renderCommunication),
		"General Practitioner": patient.generalPractitioner.map(renderPractitioner),
		"Managing Organization": patient.managingOrganization.display["@value"] ?? "_",
	};
}

export function getEncountersRecord(
	{ encounters }: PatientSummary,
): Array<TableRecord> {
	return encounters.map((encounter) => ({
		"ID": encounter.id["@value"] ?? "_",
		"Status": encounter.status["@value"] ?? "_",
		"Class": renderReference(encounter.class),
		"Types": encounter.type ? encounter.type.map(renderCodeableConcept) : "_",
		"Service Type": encounter.serviceType ? renderCodeableConcept(encounter.serviceType) : "_",
		"Participants": encounter.participant.map(renderParticipant),
		"Period": renderPeriod(encounter.period),
		"Locations": encounter.location.map((l) => renderReference(l.location)),
		"Service Provider": encounter.serviceProvider.display["@value"] ?? "_",
	}));
}

export function getCarePlansRecord(
	{ inpatient_careplans, outpatient_careplans }: PatientSummary,
): [Array<TableRecord>, Array<TableRecord>] {
	return [
		inpatient_careplans.map(renderCarePlan),
		outpatient_careplans.map(renderCarePlan),
	];
}

function renderCarePlan(careplan: CarePlan): TableRecord {
	return {
		"ID": careplan.id["@value"] ?? "_",
		"Status": careplan.status["@value"] ?? "_",
		"Intent": careplan.intent["@value"] ?? "_",
		"Category": careplan.category.text["@value"] ?? "_",
		"Period": renderPeriod(careplan.period),
		"Created": careplan.created["@value"] ?? "_",
		"Addresses": careplan.addresses ? careplan.addresses.map(renderReference) : [],
		"Goal": careplan.goal ? careplan.goal.map(renderReference) : "_",
	}
}

export function getProceduresRecord(
	{ procedures }: PatientSummary,
): Array<TableRecord> {
	return procedures.map((procedure) => ({
		"ID": procedure.id["@value"] ?? "_",
		"Status": procedure.status["@value"] ?? "_",
		"Category": procedure.category.text["@value"] ?? "_",
		"Encounter": renderReference(procedure.encounter),
		"Recorder": renderReference(procedure.recorder),
		"Location": renderReference(procedure.location),
		"Reason": procedure.reasonCode.text["@value"] ?? "_",
		"Body Site": procedure.bodySite.text["@value"] ?? "_"
	}));
}
