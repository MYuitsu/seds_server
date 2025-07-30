import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
    async POST(req, _ctx) {
        console.log("/api/advice hit.");
        const summary = await req.json();
        
        const resp = await fetch("http://127.0.0.1:3020/advice", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ summary }),
        });
        if (resp.status !== 200) {
            return new Response(
                JSON.stringify({ error: "Something went wrong with the agent." }),
                {
                    headers: {
                        "Content-Type": "application/json",
                    },
                    status: 500,
                }
            )
        }
        
        const advice = await resp.text();
        return new Response(advice, {
            status: resp.status,
            headers: {
                "Content-Type": "application/json",
            },
        });
    }
}