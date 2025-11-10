<script lang="ts">
	import Toolbar from '$lib/components/toolbar.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
</script>

<Toolbar />

<dir class="grid auto-rows-min grid-cols-2 gap-4">
	{#if data.models != undefined && data.models.length > 0}
		{#each data.models as model}
			<span>{model.id}</span>
			<span>{model.name}</span>
		{/each}
	{:else}
		<h3>Empty array</h3>
	{/if}
</dir>
