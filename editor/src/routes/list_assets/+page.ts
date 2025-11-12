import type { PageLoad } from './$types';
import { env } from '$env/dynamic/public';
import { invoke } from '@tauri-apps/api/core';

export const load: PageLoad = async () => {
	type model = {
		id: string;
		name: string;
	};

	try {
		const res = await invoke('get_model_list');
		console.log(res);

		//const models: model[] = await res.json();
		//return { models };
	} catch (e) {
		console.log(e);
	}
};
