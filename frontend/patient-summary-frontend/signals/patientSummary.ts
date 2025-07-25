import { signal, computed } from "@preact/signals";

export const selectedPatientId = signal<Patient['id'] | null>(null);
export const selectedPatient = signal<Patient | null>(null);
export const selectedEncounter = signal<Encounter | null>(null);
export const selectedObservation = signal<Observation | null>(null);
export const selectedCondition = signal<Condition | null>(null);
export const selectedStudy = signal<ImagingStudy | null>(null);
export const selectedDiagnosis = signal<DiagnosisReport | null>(null);

export const groupedObservations = computed(() => {
    const patient = selectedPatient.value;
    if (!patient) return;
    const grouped: Record<string, Record<string, Observation[]>> = {};
    for (const encounter of patient.encounters) {
        for (const obs of encounter.observations) {
            const category = obs.category[0]?.coding[0]?.display ?? "unknown";
            const code = obs.code.text;
            if (!grouped[category]) grouped[category] = {};
            if (!grouped[category][code]) grouped[category][code] = [];
            grouped[category][code].push(obs);
        }
    }
    console.log(grouped);
    return grouped;
});

export type Patient = {
    id: string;
    names: Name[];
    gender: string;
    birth_date: string;
    encounters: Encounter[];
};

export type Name = {
    use: string;
    family: string;
    given: string[];
};

export type Encounter = {
    id: string;
    status: string;
    period: Period;
    observations: Observation[];
    conditions: Condition[];
    studies: ImagingStudy[];
    diagnosis: DiagnosisReport[];
};

export type Period = {
    start: string;
    end: string;
};

export type Observation = {
    id: string;
    status: string;
    category: Coding[];
    code: Text;
    effectiveDateTime: string;
    issued: string;
    component: Component[];
    valueQuantity?: Quantity;
};

export type Component = {
    code: Text;
    valueQuantity?: Quantity;
};

export type Coding = {
    coding: Display[];
};

export type Display = {
    display: string;
};

export type Text = {
    text: string;
};

export type Quantity = {
    value: number;
    unit: string;
};

export type Condition = {
    id: string;
    clinicalStatus: ConditionStatus;
    verificationStatus: ConditionStatus;
    category: Coding[];
    code: Text;
    onsetDateTime: string;
    recordedDate: string;
};

export type ConditionStatus = {
    coding: Code[];
};

export type Code = {
    code: string;
};

export type ImagingStudy = {
    id: string;
    status: string;
    started: string;
    procedureCode: Text[];
    location: Display;
};

export type DiagnosisReport = {
    id: string;
    status: string;
    category: Coding[];
    effectiveDateTime: string;
    issued: string;
    performer: Display[];
};
