<script lang="ts">
	import '../app.css';

	import { onMount } from 'svelte';

	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner/index.js';

	let { children } = $props();

	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	import BrainCircuit from 'lucide-svelte/icons/brain-circuit';
    import House from 'lucide-svelte/icons/house';

	import Spinner from '$lib/components/spinner.svelte';

	import { page } from '$app/state';

	let hideSidebar = $state(true);
	$effect(() => {
		hideSidebar = ['/login', '/register'].includes(page.url.pathname);
	});

	import { user, workspaces, selectedWorkspace, sessions, selectedSession } from '$lib/store';
	import { toast } from 'svelte-sonner';
	import type { Page, PageResponse } from '$lib/models/page';
	import { get } from '$lib/request';
	import type Result from '$lib/models/result';
	import type { Workspace } from '$lib/models/workspace';
	import { beforeNavigate, goto } from '$app/navigation';

	onMount(() => {
		// Hydrate user, workspaces, and sessions on mount
        console.log('Mounting layout');
	});

	$inspect({ $user, $workspaces, $selectedWorkspace, $sessions, $selectedSession });

	let items = $state([
		{
			url: '/knowledge',
			title: 'Knowledge',
			icon: BrainCircuit,
			isActive: false,
			items: [
				{
					url: '/knowledge/technologies',
					id: 'technologies',
					title: 'Technologies'
				},
				{
					url: '/knowledge/documentation',
					id: 'documentation',
					title: 'Documentation'
				},
				{
					url: '/knowledge/structure',
					id: 'structure',
					title: 'Structure'
				},
				{
					url: '/knowledge/workflow',
					id: 'workflow',
					title: 'Git Workflow'
				}
			]
		}
	]);
</script>

<ModeWatcher />
<Toaster />

{#if $user === undefined}
	<div class="flex h-screen w-full items-center justify-center">
		<Spinner />
	</div>
{:else if hideSidebar}
	{@render children()}
{:else}
	<Sidebar.Provider>
		<AppSidebar bind:items />
		<Sidebar.Inset>
			<header class="flex h-10 shrink-0 items-center gap-2 transition-[width,height] ease-linear">
				<div class="flex items-center gap-2 px-3">
					<Sidebar.Trigger class="-ml-1" />
					<Separator orientation="vertical" class="mr-2 h-4" />
					<Breadcrumb.Root>
						<Breadcrumb.List>
                            <Breadcrumb.Item class="hidden md:block">
                                <Breadcrumb.Link href="/">
                                    Home
                                </Breadcrumb.Link>
                            </Breadcrumb.Item>
					        {#if $selectedWorkspace}
                                <Breadcrumb.Separator class="hidden md:block" />
							    <Breadcrumb.Item class="hidden md:block">
							    	<Breadcrumb.Link href={`/workspace/${$selectedWorkspace.id}`}
							    		>{$selectedWorkspace.name}</Breadcrumb.Link
							    	>
							    </Breadcrumb.Item>
							    {#if $selectedSession}
							        <Breadcrumb.Separator class="hidden md:block" />
							        <Breadcrumb.Item>
							        	<Breadcrumb.Link
							        		href={`/workspace/${$selectedWorkspace.id}/session/${$selectedSession.id}`}
							        	>
							        		{$selectedSession.title}
							        	</Breadcrumb.Link>
							        </Breadcrumb.Item>
							    {/if}
					        {/if}
					    </Breadcrumb.List>
					</Breadcrumb.Root>
				</div>
			</header>
			<div class="flex flex-1 flex-col gap-4 p-2 pt-0">
				{@render children()}
			</div>
		</Sidebar.Inset>
	</Sidebar.Provider>
{/if}
