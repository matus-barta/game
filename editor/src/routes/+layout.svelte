<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import AppSidebar, { type Data } from '$lib/components/app-sidebar.svelte';
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
		<div class="p-4 pt-2 pl-2">
			{@render children?.()}
		</div>
	</Sidebar.Inset>
</Sidebar.Provider>
