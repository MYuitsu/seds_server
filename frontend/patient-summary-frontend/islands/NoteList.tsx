import { useEffect, useState } from "preact/hooks";

type NoteItem = {
	order: number;
	note: string;
};

export default function NoteList() {
	const [notes, setNotes] = useState<NoteItem[]>([]);
	const [page, setPage] = useState(1);
	const [total, setTotal] = useState(0);
	const [isLoading, setIsLoading] = useState(false);
	const size = 2;

	useEffect(() => {
		const fetchNotes = async () => {
			setIsLoading(true);
			try {
				const res = await fetch(`/api/notes/${page}/${size}`);
				const data = await res.json();

				const formattedNotes: NoteItem[] = data.notes.map(
					(note: string, index: number) => ({
						order: (page - 1) * size + index + 1,
						note,
					})
				);

				setNotes(formattedNotes);
				setTotal(data.total);
			} catch (err) {
				console.error("Failed to fetch notes:", err);
			} finally {
				setIsLoading(false);
			}
		};
		fetchNotes();
	}, [page]);

	const totalPages = Math.ceil(total / size);

	return (
		<div className="max-w-6xl mx-auto p-6">
			<h2 className="text-2xl font-semibold mb-4 flex items-center gap-4">
				SOAP Notes - Page {page}
				{isLoading && (
					<span className="text-sm text-gray-500 italic animate-pulse">
						Loading...
					</span>
				)}
			</h2>

			<table className="w-full table-auto border border-collapse border-gray-300">
				<thead className="bg-gray-100">
					<tr>
						<th className="border px-4 py-2 text-left">No.</th>
						<th className="border px-4 py-2 text-left">SOAP Note</th>
					</tr>
				</thead>
				<tbody>
					{notes.map((item) => (
						<tr key={item.order} className="hover:bg-gray-50">
							<td className="border px-4 py-2 align-top">
								{item.order}
							</td>
							<td className="border px-4 py-2 whitespace-pre-wrap align-top">
								{item.note
                                    .replace(/\\n/g, "\n")
                                    .split("\n")
                                    .map((line) => (
                                        <p className="mb-2">
                                            {line}
                                        </p>
                                ))}
							</td>
						</tr>
					))}
				</tbody>
			</table>

			{/* Pagination */}
			<div className="flex justify-between mt-4">
				<button
					type="button"
					className="px-4 py-2 bg-blue-600 text-white rounded disabled:opacity-50"
					onClick={() => setPage((p) => Math.max(1, p - 1))}
					disabled={page === 1 || isLoading}
				>
					Previous
				</button>
				<span className="text-sm text-gray-700 self-center">
					Page {page} of {totalPages}
				</span>
				<button
					type="button"
					className="px-4 py-2 bg-blue-600 text-white rounded disabled:opacity-50"
					onClick={() => setPage((p) => p + 1)}
					disabled={page >= totalPages || isLoading}
				>
					Next
				</button>
			</div>
		</div>
	);
}
