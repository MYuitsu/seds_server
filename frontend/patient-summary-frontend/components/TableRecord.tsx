import { TableRecord } from "../models/summary.ts";

type TableDataProps = {
	title: string;
	data: TableRecord;
};

export default function TableData({ title, data }: Readonly<TableDataProps>) {
	return (
		<table className="w-full text-left border-collapse">
			<thead className="bg-orange-300 text-gray-700">
				<tr>
					<th colSpan={2}>{title}</th>
				</tr>
			</thead>
			<tbody>
				{Object.entries(data).map(([key, value]) => (
					<tr key={key} className="border-b border-gray-300">
						<td>{key}</td>
						<td>
							{(() => {
								if (typeof value === "string") {
									return value;
								} else if (Array.isArray(value)) {
									return (
										<ul>
											{value.map((item) => (
												<li key={item.toString()}>
													{item}
												</li>
											))}
										</ul>
									);
								} else {
									return "Invalid Data Format Or No Data";
								}
							})()}
						</td>
					</tr>
				))}
			</tbody>
		</table>
	);
}
