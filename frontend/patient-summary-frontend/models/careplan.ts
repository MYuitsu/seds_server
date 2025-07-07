import { Period, Reference, Text } from "./shared.ts";

export type CarePlanId = {
	"@value": string;
}

export type CarePlanCategory = {
	text: Text;
}

export type CarePlan = {
	id: CarePlanId;
	status: Text;
	intent: Text;
	category: CarePlanCategory;
	period: Period;
	created: Text;
	addresses: Reference[] | null;
	goal: Reference[] | null;
};
