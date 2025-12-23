<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Input } from '$lib/components/ui/input/index.js';
	import Label from '$lib/components/ui/label/label.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { Asset } from '$lib/types';

	import { open } from '@tauri-apps/plugin-dialog';

	let file = $state('Please select the file!');

	async function selectFile() {
		const _file = await open({
			multiple: false,
			directory: false,
			filters: [
				{
					name: 'Model',
					extensions: ['glb']
				}
			]
		});
		if (_file != null) file = _file;

		console.log(file);
	}

	const handleSubmit = async () => {
		const res = await postFile(file);
		console.log(res);
	};

	async function postFile(path: string) {
		try {
			const res = await invoke('post_asset', { path })
				.then((message) => {
					console.log(message);
					return message as Asset[];
				})
				.catch((error) => console.error(error));

			return { asset: res };
		} catch (e) {
			console.log(e);
		}
	}
</script>

<div class="flex flex-col gap-2">
	<Label for="file">File to upload</Label>
	<Label
		class="'flex dark:bg-input/30', 'focus-visible:border-ring focus-visible:ring-ring/50', 'aria-invalid:border-destructive dark:aria-invalid:ring-destructive/40' h-9 w-fit min-w-0 rounded-md border border-input bg-transparent px-3 pt-1.5 text-sm font-medium shadow-xs ring-offset-background transition-[color,box-shadow] outline-none selection:bg-primary
			selection:text-primary-foreground placeholder:text-muted-foreground focus-visible:ring-[3px]
			disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:ring-destructive/20"
		id="file">{file}</Label
	>

	<span class="text-sm text-muted-foreground">Opens file dialog</span>
	<Button type="submit" class="max-w-fit" onclick={selectFile}>Select file</Button>

	<span class="text-sm text-muted-foreground">Upload the model</span>
	<Button type="submit" class="max-w-fit" onclick={handleSubmit}>Submit</Button>
</div>
