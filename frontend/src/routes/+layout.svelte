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

	import Spinner from '$lib/components/spinner.svelte';

	import { fetchUser } from '$lib/api/user';
	import { fetchAllWorkspaces, fetchRecentWorkspace } from '$lib/api/workspace';
	import { fetchAllSessions } from '$lib/api/session';

	import { page } from '$app/state';

	let hideSidebar = $state(true);
	$effect(() => {
		hideSidebar = ['/login', '/register'].includes(page.url.pathname);
	});

	import {
		avatar,
		user,
		workspaces,
		selectedWorkspace,
		sessions,
		selectedSession,
		breadcrumbs
	} from '$lib/store';
	import { toast } from 'svelte-sonner';
	import type { Page } from '$lib/models/page';
	import { get } from '$lib/request';
	import type Result from '$lib/models/result';
	import type { Workspace } from '$lib/models/workspace';
	import { beforeNavigate, goto } from '$app/navigation';

	async function hydrate() {
		// User
		if (!$user) {
			let result = await fetchUser();
			user.set(result.data);
			if (result.error) {
				toast.error(result.error);
				return;
			}
		}
		// Avatar
		if (!$avatar) {
			let result = await get<Result<string | null>>('v1/me/avatar');
			if (result.error) {
				toast.error(result.error);
				return;
			}
			avatar.set(result.data);
		}
		// Workspaces
		let result = await get<Result<Workspace[]>>(`v1/me/workspaces`, {
			page: 1,
			size: 10
		});
		if (result.error) {
			toast.error(result.error);
			workspaces.set([]);
		} else {
			workspaces.set(result.data);
		}
		// Selected workspace
		// TODO: Implement proper recent workspace logic
		if (!$workspaces || $workspaces.length == 0) {
			goto('/workspaces');
			return;
		}
		selectedWorkspace.set($workspaces[0]);
		if (!$selectedWorkspace) {
			toast.error('Failed to fetch selected workspace');
			return;
		}
		// Sessions
		let page: Page = {
			page: 1,
			size: 10
		};
		sessions.set((await fetchAllSessions($selectedWorkspace.id, page)).data);
		if (!$sessions) {
			toast.error('Failed to fetch sessions');
			return;
		}
	}

	onMount(hydrate);
    beforeNavigate(hydrate);

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
			<header
				class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
			>
				<div class="flex items-center gap-2 px-4">
					<Sidebar.Trigger class="-ml-1" />
					<Separator orientation="vertical" class="mr-2 h-4" />
					{#if $selectedWorkspace}
						<Breadcrumb.Root>
							<Breadcrumb.List>
								<Breadcrumb.Item class="hidden md:block">
									<Breadcrumb.Link>{$selectedWorkspace.name}</Breadcrumb.Link>
								</Breadcrumb.Item>
								{#if $breadcrumbs}
									{#each $breadcrumbs as breadcrumb}
										<Breadcrumb.Separator class="hidden md:block" />
										<Breadcrumb.Item>
											<Breadcrumb.Link>{breadcrumb}</Breadcrumb.Link>
										</Breadcrumb.Item>
									{/each}
								{/if}
							</Breadcrumb.List>
						</Breadcrumb.Root>
					{/if}
				</div>
			</header>
			<div class="flex flex-1 flex-col gap-4 p-4 pt-0">
				{@render children()}
			</div>
		</Sidebar.Inset>
	</Sidebar.Provider>
{/if}
