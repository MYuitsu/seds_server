import { CodeableConcept, Reference, renderCodeableConcept } from "./shared.ts";

export type ReferenceOrDisplay = {
	text: string | null;
};

export function renderReferenceOrDisplay(ref: ReferenceOrDisplay): string {
	return ref.text || "Unknown";
}

export type ProvenanceAgent = {
	who: Reference;
	onBehalfOf: ReferenceOrDisplay | null;
};

export function renderProvenanceAgent(agent: ProvenanceAgent): string {
	const whoDisplay = agent.who.display || "Unknown Agent";
	const onBehalfOfDisplay = agent.onBehalfOf
		? agent.onBehalfOf.text || "Unknown On Behalf Of"
		: "N/A";
	return `${whoDisplay} (On Behalf Of: ${onBehalfOfDisplay})`;
}

export type Provenance = {
	resourceType: "Provenance";
	id: string;
	target: Reference[];
	recorded: string | null;
	agent: ProvenanceAgent[];
};

export type Annotation = {
	text: string | null;
};

export function renderAnnotation(annotation: Annotation): string {
	return annotation.text || "No Annotation";
}

export type BodySiteExtension = {
	url: string;
	value_codeable_concept: CodeableConcept | null;
};

export function renderBodySiteExtension(extension: BodySiteExtension): string {
	if (extension.value_codeable_concept) {
		return renderCodeableConcept(extension.value_codeable_concept);
	}

	return "Unknown Body Site";
}

export type ProcedureBodySite = {
	extension: BodySiteExtension[];
};

export function renderProcedureBodySite(bodySite: ProcedureBodySite): string {
	return bodySite.extension.map(renderBodySiteExtension).join(", ") ??
		"Unknown Body Site";
}

export type Procedure = {
	resourceType: "Procedure";
	id: string;
	status: string | null;
	category: CodeableConcept | null;
	code: CodeableConcept | null;
	subject: Reference;
	performedDateTime: string | null;
	body_site: ProcedureBodySite[];
	note: Annotation[];
};
