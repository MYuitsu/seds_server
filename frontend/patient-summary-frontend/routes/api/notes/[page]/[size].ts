import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
    async GET(_req, ctx) {
        const page = Number(ctx.params.page);
        const size = Number(ctx.params.size);
        const resp = await fetch(`http://127.0.0.1:3020/notes?page=${page}&size=${size}`);
        if (resp.status !== 200) {
            console.error("Cannot generate notes.");
            return new Response(
                JSON.stringify({ error: "Internal error" }),
                { status: 500, headers: { "Content-Type": "application/json" } },
            );
        }

        const data = await resp.text();
        return new Response(data, {
            status: 200,
            headers: {
                "Content-Type": "application/json"
            }
        })
    }
}