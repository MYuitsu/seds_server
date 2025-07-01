import { ComponentChildren } from "https://esm.sh/v128/preact@10.22.0/src/index.d.ts";

type CardLayoutProps = {
	children: ComponentChildren;
};

export default function CardLayout({ children }: Readonly<CardLayoutProps>) {
	return (
		<div className="max-w-4xl w-full p-6 bg-white rounded-lg shadow-md border border-gray-200 mb-6">
			{children}
		</div>
	);
}
