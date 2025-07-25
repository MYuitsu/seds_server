import type uPlotType from "https://esm.sh/uplot@1.6.32";
import { effect } from "@preact/signals";
import { useRef } from "preact/hooks";
import { groupedObservations } from "../signals/patientSummary.ts";
import type { AlignedData } from "https://esm.sh/uplot@1.6.32";

interface DiagramProps {
	category: string;
	code: string;
}

export default function Diagram(
	{ category, code }: Readonly<DiagramProps>,
) {
	const chartRef = useRef<HTMLDivElement>(null);
	const plotRef = useRef<uPlotType | null>(null);

	effect(() => {
		import("https://esm.sh/uplot@1.6.32").then(({ default: uPlot }) => {
			const data = groupedObservations.value?.[category];
			if (!chartRef.current || !data) return;

			const observations = data[code];
			if (!observations || observations.length === 0) return;

			const points: [number, number][] = observations
				.map((obs) => [
					new Date(obs.effectiveDateTime ?? obs.issued ?? "")
						.getTime() / 1000,
					obs.valueQuantity?.value,
				])
				.filter(([x, y]) => !!x && y !== undefined) as [
					number,
					number,
				][];

			if (points.length === 0) return;

			const timestamps = points.map(([ts]) => ts);
			const values = points.map(([, val]) => val ?? NaN);

			const alignedData: AlignedData = [
				Float64Array.from(timestamps),
				Float64Array.from(values),
			];

			plotRef.current?.destroy();

			plotRef.current = new uPlot(
				{
					width: 400,
					height: 200,
					title: code,
					scales: { x: { time: true }, y: { auto: true } },
					axes: [
						{
							stroke: "#333",
							grid: { stroke: "#ddd" },
							font: "12px sans-serif",
						},
						{
							stroke: "#333",
							grid: { stroke: "#ddd" },
							font: "12px sans-serif",
						},
					],
					series: [
						{}, // x-axis
						{
							label: code,
							stroke: "steelblue",
							width: 2,
							points: { show: true },
						},
					],
					legend: { show: false }, // external legend recommended
				},
				alignedData,
				chartRef.current,
			);
		});
	});

	return <div ref={chartRef} />;
}
