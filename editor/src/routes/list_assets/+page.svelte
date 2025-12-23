<script lang="ts">
	import Toolbar from '$lib/components/toolbar.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
</script>

<Toolbar />

{#if data.asset != undefined && data.asset.length > 0}
	<table class="w-full table-auto">
		<thead class="bg-accent">
			<tr>
				<th class="px-2 text-left">ID</th>
				<th class="px-2 text-left">Name</th>
			</tr>
		</thead>
		<tbody>
			{#each data.asset as asset}
				<tr>
					<td class="px-2">
						{asset.id}
					</td>
					<td class="px-2">
						{asset.name}
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
{:else}
	<h3>Empty array</h3>
{/if}
