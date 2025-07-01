export type HumanName = {
	text: string | null;
};

export function renderHumanName(name: HumanName): string {
	return name.text || "Unknown Name";
}

export type Address = {
	city: string | null;
	state: string | null;
	country: string | null;
};

export function renderAddress(address: Address): string {
	const parts = [];
	if (address.city) parts.push(address.city);
	if (address.state) parts.push(address.state);
	if (address.country) parts.push(address.country);
	return parts.join(", ") ?? "Unknown Address";
}

export type Patient = {
	id: string;
	name: HumanName[];
	gender: string | null;
	birthDate: string | null;
	address: Address[];
};
