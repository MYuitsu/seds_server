import { Coding, Reference } from "./shared.ts";

export type Attachment = {
	contentType: string | null;
	url: string | null;
	title: string | null;
};

export function renderAttachment(attachment: Attachment): string {
	if (attachment.url) {
		return attachment.title
			? `${attachment.title} (${attachment.url})`
			: attachment.url;
	}

	return attachment.title || "No Attachment";
}

export type Quantity = {
	value: number | null;
	unit: string | null;
	system: string | null;
	code: string | null;
};

export function renderQuantity(quantity: Quantity): string {
	if (quantity.value !== null && quantity.unit) {
		return `${quantity.value} ${quantity.unit}`;
	}

	return "Unknown Quantity";
}

export type QuestionnaireAnswerValue =
	| { valueBoolean: boolean }
	| { valueDecimal: number }
	| { valueString: string }
	| { valueDate: string }
	| { valueTime: string }
	| { valueCoding: Coding }
	| { valueQuantity: Quantity }
	| { valueAttachment: Attachment };

export function renderQuestionnaireAnswerValue(
	value: QuestionnaireAnswerValue,
): string {
	if ("valueBoolean" in value) {
		return value.valueBoolean ? "True" : "False";
	} else if ("valueDecimal" in value) {
		return value.valueDecimal.toString();
	} else if ("valueString" in value) {
		return value.valueString || "No String Value";
	} else if ("valueDate" in value) {
		return value.valueDate || "No Date Value";
	} else if ("valueTime" in value) {
		return value.valueTime || "No Time Value";
	} else if ("valueCoding" in value) {
		return value.valueCoding.display || "No Coding Value";
	} else if ("valueQuantity" in value) {
		return renderQuantity(value.valueQuantity);
	} else if ("valueAttachment" in value) {
		return renderAttachment(value.valueAttachment);
	}

	return "Unknown Value Type";
}

export type QuestionnaireAnswer = {
	value: QuestionnaireAnswerValue | null;
};

export function renderQuestionnaireAnswer(answer: QuestionnaireAnswer): string {
	if (answer.value) {
		return renderQuestionnaireAnswerValue(answer.value);
	}

	return "No Answer Value";
}

export type QuestionnaireItem = {
	link_id: string;
	text: string | null;
	answer: QuestionnaireAnswer[];
};

export function renderQuestionnaireItem(item: QuestionnaireItem): string {
	const answers = item.answer.map(renderQuestionnaireAnswer).join(", ");
	return `${item.link_id}: ${item.text || "No Text"} - Answers: ${answers}`;
}

export type QuestionnaireResponse = {
	id: string;
	status: string | null;
	authored: string | null;
	subject: Reference;
	item: QuestionnaireItem[];
};
