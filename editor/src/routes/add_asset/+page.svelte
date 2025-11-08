<script lang="ts">
	import type { PageData } from './$types';
	import Form from './form.svelte';
	let { data }: { data: PageData } = $props();
</script>

<Form {data} />
