import { TableRecord } from "../models/summary.ts";

type TableDataProps = {
	title: string;
	data: TableRecord;
};

export default function TableData({ title, data }: Readonly<TableDataProps>) {
	return (
		<div className="overflow-x-auto max-w-full">
			<table className="w-full text-left border-collapse table-auto">
				<thead className="bg-orange-300 text-gray-700">
					<tr>
						<th colSpan={2} className="px-4 py-2">
							{title}
						</th>
					</tr>
				</thead>
				<tbody>
					{Object.entries(data).map(([key, value]) => (
						<tr key={key} className="border-b border-gray-300">
							<td className="px-4 py-2 max-w-[200px] break-words align-top text-sm font-medium text-gray-700">
								{key}
							</td>
							<td className="px-4 py-2 max-w-[400px] break-words align-top text-sm text-gray-800">
								{(() => {
									if (typeof value === "string") {
										return value;
									} else if (Array.isArray(value)) {
										return (
											<ul className="list-disc list-inside space-y-1">
												{value.map((item) => (
													<li key={item.toString()}>
														{item}
													</li>
												))}
											</ul>
										);
									} else {
										return (
											<span className="italic text-gray-500">
												Invalid Data Format Or No Data
											</span>
										);
									}
								})()}
							</td>
						</tr>
					))}
				</tbody>
			</table>
		</div>
	);
}
