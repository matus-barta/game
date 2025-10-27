<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Input } from '$lib/components/ui/input/index.js';
	import Label from '$lib/components/ui/label/label.svelte';
	import * as Form from '$lib/components/ui/form/index.js';
	import { formSchema } from './schema';
	import { superForm, fileFieldProxy } from 'sveltekit-superforms';
	import { zod4 } from 'sveltekit-superforms/adapters';

	let { data } = $props();

	// const superform = superForm(data.form, {
	// 	validators: zod4(formSchema)
	// });
	// const { form, enhance } = superform;

	// const { value, constraints, errors } = fileFieldProxy(superform, 'file');

	const form = superForm(data.form, {
		validators: zod4(formSchema)
	});

	const { form: formData, enhance, message } = form;

	const { value, constraints, errors } = fileFieldProxy(form, 'file');

	$effect(() => {
		if ($message) console.log($message);
		if ($errors) $errors.forEach((err) =>{console.error(err)});
	});
</script>

<!-- <form class="flex flex-col space-y-2" method="POST" enctype="multipart/form-data" use:enhance>
	<Label for="file">Select file to upload</Label>
	<Input aria-invalid={$errors ? 'true' : undefined} id="file" type="file" name="file" bind:files={$value} accept="model/gltf-binary" {...$constraints}/>

	{#if $errors}<span class="text-sm tracking-tight text-destructive">{$errors}</span>{/if}
	<Button class="max-w-fit" type="submit">Submit</Button>
</form> -->

<form class="flex flex-col space-y-2" method="POST" enctype="multipart/form-data" use:enhance>
	<Form.Field {form} name="file">
		<Form.Control>
			<Form.Label>File to upload</Form.Label>
			<Input
				type="file"
				name="file"
				bind:files={$value}
				accept="model/gltf-binary"
				{...$constraints}
			/>
		</Form.Control>
		{#if $message}
			<Form.Description>{$message}</Form.Description>
		{:else}
			<Form.Description>Here upload the model</Form.Description>
		{/if}

		<Form.FieldErrors />
	</Form.Field>
	<Form.Button class="max-w-fit">Submit</Form.Button>
</form>
