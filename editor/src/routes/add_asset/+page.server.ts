import type { PageServerLoad, Actions } from './$types';
import { superValidate, fail, message } from 'sveltekit-superforms';
import { zod4 } from 'sveltekit-superforms/adapters';
import { formSchema } from './schema';
import { withFiles } from 'sveltekit-superforms';

import { env } from '$env/dynamic/public';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod4(formSchema))
	};
};

// export const actions: Actions = {
// 	default: async ({ request, fetch }) => {
// 		const form = await superValidate(request, zod4(formSchema));
// 		if (!form.valid) {
// 			console.log('Form invalid:', form.errors);
// 			return fail(400, withFiles({ form }));
// 		}

// 		const contentType = request.headers.get('content-type') || '';
// 		//const contentLength = request.headers.get('content-length') || '';

// 		console.log('content-length:', contentType);

// 		const data = await fetch(`http://${env.PUBLIC_ASSET_SERVER_URL}/dev/model`, {
// 			method: 'POST',
// 			headers: {
// 				'Content-Type': contentType
// 				//'Content-Length': contentLength
// 			},
// 			body: form.data.file
// 		})
// 			.then((res) => {
// 				return res.text();
// 			})
// 			.catch((e) => {
// 				console.log(e);
// 			});

// 		// if (!res.ok) {
// 		// 	setError(form, data);
// 		// 	return { form };
// 		// }

// 		console.log('message:', data);

// 		return message(form, data);
// 	}
// };
