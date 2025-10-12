<script lang="ts">
	import VersionSwitcher from './version-switcher.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { page } from '$app/state';

	export interface Data {
		navMain: {
			title: string;
			items: { title: string; url: string }[];
		}[];
	}

	interface Props {
		data: Data;
	}

	let { data }: Props = $props();

	function isApp(title: string) {
		return page.url.pathname.includes(title);
	}
</script>

<Sidebar.Root>
	<Sidebar.Header>
		<VersionSwitcher />
	</Sidebar.Header>
	<Sidebar.Content>
		<!-- We create a Sidebar.Group for each parent. -->
		{#each data.navMain as group (group.title)}
			<Sidebar.Group>
				<Sidebar.GroupLabel>{group.title}</Sidebar.GroupLabel>
				<Sidebar.GroupContent>
					<Sidebar.Menu>
						{#each group.items as item (item.title)}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton isActive={isApp(item.url)}>
									{#snippet child({ props })}
										<a href={item.url} {...props}>{item.title}</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/each}
					</Sidebar.Menu>
				</Sidebar.GroupContent>
			</Sidebar.Group>
		{/each}
	</Sidebar.Content>
	<Sidebar.Rail />
</Sidebar.Root>
