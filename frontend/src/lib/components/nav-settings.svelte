<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';

	import { selectedWorkspace } from '$lib/store';

	let {
		items = $bindable(),
        ...props
	}: {
		items: {
			title: string;
			callback?: () => void;
			icon?: any;
		}[];
	} = $props();
</script>

{#if $selectedWorkspace}
	<Sidebar.Group>
		<Sidebar.GroupLabel>Mirabel</Sidebar.GroupLabel>
		<Sidebar.Menu>
			{#each items as mainItem (mainItem.title)}
				<Sidebar.MenuItem {...props}>
					<Sidebar.MenuButton {...props} onclick={mainItem.callback}>
						{#snippet tooltipContent()}
							{mainItem.title}
						{/snippet}
						{#if mainItem.icon}
							<mainItem.icon />
						{/if}
						<span>{mainItem.title}</span>
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			{/each}
		</Sidebar.Menu>
	</Sidebar.Group>
{/if}
