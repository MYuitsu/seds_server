import PatientViewer from "../../islands/PatientViewer.tsx";

export default function PatientSummaryPage() {
	return (
		<div className="flex flex-col min-h-screen">
			{/* Navigation Bar */}
			<nav className="bg-orange-200 text-black px-6 py-4 shadow">
				<div className="max-w-6xl mx-auto flex justify-between items-center">
					<div className="text-lg font-semibold">Medical Agent</div>
					<div className="space-x-4">
						<a href="/" className="hover:underline">Home</a>
						<a href="/patientsummary" className="hover:underline">Patient Summary</a>
						<a href="/soapnotes" className="hover:underline">SOAP Notes</a>
					</div>
				</div>
			</nav>

			{/* Main Content */}
			<main className="flex-1 flex">
				<PatientViewer />
			</main>
		</div>
	);
}
