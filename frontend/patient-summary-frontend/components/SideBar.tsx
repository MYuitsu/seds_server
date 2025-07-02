type Props = {
	ids: string[];
	onSelect: (id: string) => void;
};

export default function SideBar({ ids, onSelect }: Readonly<Props>) {
	return (
		<aside className="w-1/4 p-4 bg-gray-100 h-screen overflow-y-auto">
			<ul>
				{ids.map((id) => (
					<li
						key={id}
						className="max-w-4xl w-full p-6 bg-white rounded-lg shadow-md border border-gray-200 mb-6"
					>
						<button
							type="button"
							onClick={() => onSelect(id)}
							className="block w-full h-full"
						>
							Patient-{id}
						</button>
					</li>
				))}
			</ul>
		</aside>
	);
}
