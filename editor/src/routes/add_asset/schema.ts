import { z } from 'zod';

export const formSchema = z.object({
	file: z.file().mime('model/gltf-binary')
});

export type FormSchema = typeof formSchema;
