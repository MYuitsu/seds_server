export type Reference = {
	reference: string | null;
	display: string | null;
};

export function renderReference(ref: Reference): string {
	return ref.display || "Unknown Reference";
}

export type Period = {
	start: string | null;
	end: string | null;
};

export function renderPeriod(period: Period): string {
	const start = period.start
		? new Date(period.start).toLocaleDateString()
		: "Unknown Start";
	const end = period.end
		? new Date(period.end).toLocaleDateString()
		: "Unknown End";
	return `${start} - ${end}`;
}

export type Coding = {
	system: string | null;
	code: string | null;
	display: string | null;
};

export function renderCoding(coding: Coding): string {
	return coding.display ||
		`${coding.code || "Unknown"} (${coding.system || "Unknown System"})`;
}

export type CodeableConcept = {
	coding: Coding[];
	text: string | null;
};

export function renderCodeableConcept(concept: CodeableConcept): string {
	if (concept.text) {
		return concept.text;
	}

	if (concept.coding && concept.coding.length > 0) {
		return concept.coding.map(renderCoding).join(", ");
	}

	return "Unknown";
}
