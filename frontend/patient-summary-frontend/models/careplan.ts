import { CodeableConcept, Period, Reference } from "./shared.ts";

export type CarePlanActivityDetail = {
	kind: string | null;
	code: CodeableConcept | null;
	goal: string[];
	status: string[];
	doNotPerform: boolean | null;
	scheduledPeriod: Period | null;
	performer: Reference | null;
	description: string | null;
};

export type CarePlanActivity = {
	detail: CarePlanActivityDetail | null;
};

export type CarePlan = {
	id: string;
	status: string | null;
	intent: string | null;
	category: CodeableConcept | null;
	title: string | null;
	subject: Reference;
	period: Period | null;
	created: string | null;
	author: Reference | null;
	goal: Reference[];
	activity: CarePlanActivity[];
};
