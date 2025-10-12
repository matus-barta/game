import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	interface model {
		id: string;
		name: string;
	}

	const res = await fetch('http://localhost:3000/dev/model');
	const models: model[] = await res.json();

	return { models };
};
