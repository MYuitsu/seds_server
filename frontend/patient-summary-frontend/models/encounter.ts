import { CodeableConcept, Period, Reference, renderCodeableConcept, renderReference, Text } from "./shared.ts";

export type EncounterId = {
	"@value": string;
}

export type Participant = {
    type: CodeableConcept | null;
    individual: ParticipantIndividual;
}

export function renderParticipant(participant: Participant): string {
    const strType = participant.type ? renderCodeableConcept(participant.type) : "unknown";
    const strIndividual = participant.individual.display["@value"] ?? "anonymous";

    return `${strIndividual} (${strType})`;
}

export type ParticipantIndividual = {
    type: CodeableConcept | null;
    display: Text;
}

export type EncounterLocation = {
    location: Reference;
}

export type ServiceProvider = {
    display: Text;
}

export type Encounter = {
	id: EncounterId,
    status: Text,
    class: Reference,
    type: CodeableConcept[] | null,
    serviceType: CodeableConcept | null,
    participant: Participant[],
    period: Period,
    location: EncounterLocation[],
    serviceProvider: ServiceProvider,
};
