<script lang="ts">
	import Toolbar from '$lib/components/toolbar.svelte';
	import type { PageData } from './$types';
	import Form from './form.svelte';
	let { data }: { data: PageData } = $props();
</script>

<Toolbar />
<Form {data} />
