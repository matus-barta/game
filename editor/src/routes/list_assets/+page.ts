import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/core';
import type { Asset } from '$lib/types';

export const load: PageLoad = async () => {
	try {
		const res = await invoke('get_model_list')
			.then((message) => {
				console.log(message);
				return message as Asset[];
			})
			.catch((error) => console.error(error));

		return { asset: res };
	} catch (e) {
		console.log(e);
	}
};
