import { Period, Reference, Text } from "./shared.ts";

export type ProcedureId = {
	"@value": string;
}

export type ProcedureCategory = {
	text: Text;
}

export type ProcedureReason = {
	text: Text;
}

export type BodySite = {
	text: Text;
}

export type Procedure = {
	id: ProcedureId;
	status: Text;
	category: ProcedureCategory;
	encounter: Reference;
	performedPeriod: Period;
	recorder: Reference;
	location: Reference;
	reasonCode: ProcedureReason;
	bodySite: BodySite;
};
