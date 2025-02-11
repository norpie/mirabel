<script lang="ts">
	import NavSettings from '$lib/components/nav-settings.svelte';
	import NavChats from '$lib/components/nav-chats.svelte';
	import NavUser from '$lib/components/nav-user.svelte';
	import WorkspaceSwitcher from '$lib/components/workspace-switcher.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	let {
		items = $bindable(),

		ref = $bindable(null),
		collapsible = 'icon',
		...restProps
	}: {
		items: {
			title: string;
			url: string;
			icon: any;
			isActive: boolean;
			items: {
				title: string;
				url: string;
			}[];
		}[];

		ref?: any;
		collapsible?: 'offcanvas' | 'icon' | 'none';
	} = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
	<Sidebar.Header>
		<WorkspaceSwitcher />
	</Sidebar.Header>
	<Sidebar.Content>
		<NavSettings {items} />
		<NavChats />
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavUser />
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
