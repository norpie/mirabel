<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	import BrainCircuit from 'lucide-svelte/icons/brain-circuit';
	import Settings from 'lucide-svelte/icons/settings';

	import { selectedWorkspace } from '$lib/store';

	let { ...props }: {} = $props();

	const baseLinks = [
		{
			slug: 'settings',
			title: 'Settings',
			icon: Settings
		},
		{
			slug: 'knowledge',
			title: 'Knowledge',
			icon: BrainCircuit
		}
	];

	let linkBase = $derived(`/workspace/${$selectedWorkspace?.id}`);

	let links = $derived.by(() => {
		return baseLinks.map((link) => ({
			...link,
			href: linkBase + '/' + link.slug
		}));
	});
</script>

{#if $selectedWorkspace}
	<Sidebar.Group>
		<Sidebar.GroupLabel>Mirabel</Sidebar.GroupLabel>
		<Sidebar.Menu>
			{#each links as mainItem (mainItem.title)}
				<Sidebar.MenuItem {...props}>
					<Sidebar.MenuButton {...props} class="p-0">
						<a href={mainItem.href} class="flex items-center gap-2 p-2 h-full w-full">
							{#if mainItem.icon}
								<mainItem.icon class="h-4 w-4" />
							{/if}
							<span>{mainItem.title}</span>
						</a>
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			{/each}
		</Sidebar.Menu>
	</Sidebar.Group>
{/if}
