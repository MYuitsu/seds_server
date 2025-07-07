import { Coding, renderCoding, Text } from "./shared.ts";

export type PatientId = {
	"@value": string;
}

export type HumanName = {
	usage: Text | null;
	text: Text | null;
};

export function renderHumanName(name: HumanName): string {
	const use = name.usage ? name.usage["@value"] : "";
	const text = name.text ? name.text["@value"] : "Unknown";

	return use.length === 0 ? text : `${text} (${use})`;
}

export type Address = {
	city: Text | null;
	state: Text | null;
	country: Text | null;
};

export function renderAddress(address: Address): string {
	const parts = [];
	if (address.city) parts.push(address.city["@value"]);
	if (address.state) parts.push(address.state["@value"]);
	if (address.country) parts.push(address.country["@value"]);
	return parts.join(", ") ?? "Unknown Address";
}

export type PatientMaritalStatus = {
	text: Text;
}

export type PatientCommunication = {
	language: PatientLanguage;
}

export type PatientLanguage = {
	text: Text;
}

export type CommunicationWay = {
	value: BoolVal;
	extensions: Extension[];
}

export type BoolVal = {
	value: boolean;
}

export type Extension = {
	valueCoding: Coding;
}

export function renderCommunication(communication: PatientCommunication): string {
	return `${communication.language.text["@value"] ?? "_"}`;
}

export type Practitioner = {
	type: Text;
	display: Text;
}

export function renderPractitioner(practitioner: Practitioner): string {
	const strType = practitioner.type["@value"] ?? "unknown";
	const strDisplay = practitioner.display["@value"] ?? "anonymous";

	return `${strDisplay} (${strType})`;
}

export type Organization = {
	display: Text;
}

export type Patient = {
	id: PatientId;
	name: HumanName[];
	gender: Text | null;
	birthDate: Text | null;
	address: Address[];
	maritalStatus: PatientMaritalStatus,
    communication: PatientCommunication[],
    generalPractitioner: Practitioner[],
    managingOrganization: Organization,
};
