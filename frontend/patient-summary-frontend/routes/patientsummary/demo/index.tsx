import { Handlers, PageProps } from "$fresh/server.ts";
import SummaryMainContent from "../../../islands/SummaryMainContent.tsx";
import SummarySideBar from "../../../islands/SummarySideBar.tsx";
import { Patient } from "../../../signals/patientSummary.ts";

export const handler: Handlers = {
    async GET(_req, ctx) {
        const resp = await fetch("http://127.0.0.1:3010/demo/patients", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        });
        if (resp.status !== 200) {
            return ctx.render({ patients: [] });
        }
        const patients: Pick<Patient, 'id' | 'names'>[] = await resp.json()
            .catch((err) => {
                console.log(err);
                return [];
            });
        return ctx.render({ patients });
    }
}

export default function DemoSummary(props: PageProps<{ patients: Pick<Patient, 'id' | 'names'>[] }>) {
    return (
        <div className="flex h-screen">
            <SummarySideBar patients={props.data.patients}/>
            <SummaryMainContent />
        </div>
    );
}