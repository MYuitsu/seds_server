import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
	GET(_req, _ctx) {
		const patientIds = [
			"erXuFYUfucBZaryVksYEcMg3",
			"eq081-VQEgP8drUUqCWzHfw3",
			"eAB3mDIBBcyUKviyzrxsnAw3",
			"egqBHVfQlt4Bw3XGXoxVxHg3",
			"eIXesllypH3M9tAA5WdJftQ3",
			"eh2xYHuzl9nkSFVvV3osUHg3",
			"e0w0LEDCYtfckT6N.CkJKCw3",
		];

		return new Response(JSON.stringify(patientIds), {
			headers: {
				"Content-Type": "application/json",
			},
			status: 200,
		});
	},
};
