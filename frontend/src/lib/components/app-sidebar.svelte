<script lang="ts">
	import NavSettings from '$lib/components/nav-settings.svelte';
	import NavSessions from '$lib/components/nav-sessions.svelte';
	import NavUser from '$lib/components/nav-user.svelte';
	import WorkspaceSwitcher from '$lib/components/workspace-switcher.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	let {
		ref = $bindable(null),
		collapsible = 'icon',
		...restProps
	}: {
		ref?: any;
		collapsible?: 'offcanvas' | 'icon' | 'none';
	} = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
	<Sidebar.Header>
		<WorkspaceSwitcher />
	</Sidebar.Header>
	<Sidebar.Content>
		<NavSettings />
		<NavSessions />
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavUser />
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
