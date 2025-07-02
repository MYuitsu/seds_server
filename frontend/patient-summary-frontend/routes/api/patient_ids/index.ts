import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
	GET(_req, _ctx) {
		const patientIds = [
			"a1",
			"b2",
			"c3",
			"d4",
			"e5",
			"f6",
			"g7",
			"h8",
			"i9",
			"j10",
			"k11",
			"l12",
			"m13",
			"n14",
		];

		return new Response(JSON.stringify(patientIds), {
			headers: {
				"Content-Type": "application/json",
			},
			status: 200,
		});
	},
};
