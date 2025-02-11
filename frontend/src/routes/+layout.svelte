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
	import { fetchAllProjects, fetchRecentProject } from '$lib/api/project';
	import { fetchAllChats } from '$lib/api/chat';

	import { user, projects, selectedProject, chats, selectedChat, breadcrumbs } from '$lib/store';
	import { toast } from 'svelte-sonner';

	onMount(async () => {
		user.set(await fetchUser());
		if (!$user) {
			toast.error('Failed to fetch user data');
			return;
		}
		projects.set((await fetchAllProjects({ page: 1, pageSize: 10 })).data);
		if (!$projects) {
			toast.error('Failed to fetch projects');
			return;
		}
		selectedProject.set(await fetchRecentProject());
		if (!$selectedProject) {
			toast.error('Failed to fetch selected project');
			return;
		}
		chats.set((await fetchAllChats($selectedProject.id, { page: 1, pageSize: 10 })).data);
		if (!$chats) {
			toast.error('Failed to fetch chats');
			return;
		}
		selectedChat.set($chats[0]);
		if (!$selectedChat) {
			toast.error('Failed to fetch selected selected chat');
			return;
		}
	});

	$inspect({ $user, $projects, $selectedProject, $chats, $selectedChat });

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

{#if $user && $projects}
	<Sidebar.Provider>
		<AppSidebar bind:items />
		<Sidebar.Inset>
			<header
				class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
			>
				<div class="flex items-center gap-2 px-4">
					<Sidebar.Trigger class="-ml-1" />
					<Separator orientation="vertical" class="mr-2 h-4" />
					{#if $selectedProject}
						<Breadcrumb.Root>
							<Breadcrumb.List>
								<Breadcrumb.Item class="hidden md:block">
									<Breadcrumb.Link>{$selectedProject.name}</Breadcrumb.Link>
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
			{@render children()}
		</Sidebar.Inset>
	</Sidebar.Provider>
{:else}
	<div class="flex h-screen w-full items-center justify-center">
		<Spinner />
	</div>
{/if}
