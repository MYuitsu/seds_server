export type Text = {
	"@value": string;
}

export type Reference = {
	reference: Text | null;
	display: Text | null;
};

export function renderReference(ref: Reference): string {
	return ref.display ? ref.display["@value"] : "Unknown Reference";
}

export type Period = {
	start: Text | null;
	end: Text | null;
};

export function renderPeriod(period: Period): string {
	const start = period.start
		? new Date(period.start["@value"]).toLocaleDateString()
		: "Unknown Start";
	const end = period.end
		? new Date(period.end["@value"]).toLocaleDateString()
		: "Unknown End";
	return `${start} - ${end}`;
}

export type Coding = {
	display: Text | null;
};

export function renderCoding(coding: Coding): string {
	return coding.display ? coding.display["@value"] : "Unknown";
}

export type CodeableConcept = {
	coding: Coding[];
	text: Text | null;
};

export function renderCodeableConcept(concept: CodeableConcept): string {
	if (concept.text !== null) {
		return concept.text["@value"];
	}

	if (concept.coding && concept.coding.length > 0) {
		return concept.coding.map(renderCoding).join(", ");
	}

	return "Unknown";
}
