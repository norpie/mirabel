<script lang="ts">
	import '../app.css';

	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner/index.js';

	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	import BrainCircuit from 'lucide-svelte/icons/brain-circuit';

	import { page } from '$app/state';

	let hideSidebar: boolean = $derived(isExcluded(page.url.pathname));

    function isExcluded(path: string): boolean {
        return ['/login', '/register'].includes(path);
    }

	import { user, workspaces, selectedWorkspace, selectedSession } from '$lib/store';
	import type { LayoutProps } from './$types';

	let { data, children }: LayoutProps = $props();

    $effect(() => {
	    user.set(data.user);
	    workspaces.set(data.workspaces);
    });

	let items = $state([
		{
			title: 'Knowledge',
			callback: () => {
				console.log('Knowledge clicked');
			},
			icon: BrainCircuit
		}
	]);
</script>

<ModeWatcher />
<Toaster richColors expand position="top-center" />

{#if hideSidebar}
	{@render children()}
{:else}
	<Sidebar.Provider class="h-full">
		<AppSidebar bind:items />
		<Sidebar.Inset>
			<header class="flex h-10 shrink-0 items-center gap-2 transition-[width,height] ease-linear">
				<div class="flex items-center gap-2 px-3">
					<Sidebar.Trigger class="-ml-1" />
					<Separator orientation="vertical" class="mr-2 h-4" />
					<Breadcrumb.Root>
						<Breadcrumb.List>
							<Breadcrumb.Item class="hidden md:block">
								<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
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
