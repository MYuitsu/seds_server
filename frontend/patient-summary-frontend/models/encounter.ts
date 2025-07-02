import { Coding, Period, Reference, renderReference } from "./shared.ts";

export type Diagnosis = {
	condition: Reference | null;
};

export function renderDiagnosis(diagnosis: Diagnosis): string {
	return diagnosis.condition
		? renderReference(diagnosis.condition)
		: "Unknown Diagnosis";
}

export type Encounter = {
	id: string;
	status: string | null;
	class: Coding | null;
	subject: Reference;
	period: Period | null;
	diagnosis: Diagnosis[] | null;
};
