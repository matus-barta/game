import type { PageLoad } from './$types';
import { env } from '$env/dynamic/public';

export const load: PageLoad = async ({ fetch }) => {
	interface model {
		id: string;
		name: string;
	}

	try {
		const res = await fetch(`http://${env.PUBLIC_ASSET_SERVER_URL}/dev/model`);

		const models: model[] = await res.json();
		return { models };
	} catch (e) {
		console.log(e);
	}
};
