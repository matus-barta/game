import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/core';

export const load: PageLoad = async () => {
	type asset = {
		id: string;
		name: string;
	};

	try {
		const res = await invoke('get_model_list')
			.then((message) => {
				console.log(message);
				return message as asset[];
			})
			.catch((error) => console.error(error));

		return { asset: res };
	} catch (e) {
		console.log(e);
	}
};
