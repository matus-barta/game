<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import AppSidebar, { type Data } from '$lib/components/app-sidebar.svelte';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	let { children } = $props();

	const navData: Data = {
		navMain: [
			{
				title: 'Asset Manager',
				items: [
					{
						title: 'List Assets',
						url: 'list_assets'
					},
					{
						title: 'Add Asset',
						url: 'add_asset'
					}
				]
			}
		]
	};
</script>

<ModeWatcher />
<Sidebar.Provider>
	<AppSidebar data={navData} />
	<Sidebar.Inset>
		<header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
			<Sidebar.Trigger class="-ml-1" />
			<Separator orientation="vertical" class="mr-2 h-4" />
		</header>
		<div class="p-4">
			{@render children?.()}
		</div>
	</Sidebar.Inset>
</Sidebar.Provider>
