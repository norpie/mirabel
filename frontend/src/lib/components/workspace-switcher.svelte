<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { useSidebar } from '$lib/components/ui/sidebar/index.js';
	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import Plus from 'lucide-svelte/icons/plus';

	import { selectedWorkspace, workspaces } from '$lib/store';
	import Spinner from './spinner.svelte';
	import * as Avatar from '$lib/components/ui/avatar/index.js';

	import { Button } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import { toast } from 'svelte-sonner';
	import { post } from '$lib/request';
	import { goto } from '$app/navigation';
	import { type Workspace } from '$lib/models/workspace';

	let localSelectedWorkspace = $derived($selectedWorkspace);

	const sidebar = useSidebar();

	let workspaceDialogOpen = $state(false);

	let workspaceName = $state('');
	let workspaceCode = $state('');

	async function createWorkspace() {
		if (!workspaceName) {
			toast.error('Workspace name is required');
			return;
		}
		let result = await post<Workspace>('v1/me/workspace', {
			name: workspaceName
		});
		if (result.error) {
			toast.error(result.error);
			return;
		}
        if (!result.data) {
            toast.error('Failed to create workspace');
            return;
        }
		workspaceDialogOpen = false;
		$workspaces.push(result.data);
        $selectedWorkspace = result.data;
		goto(`/workspace/${result.data.id}`, {
			invalidateAll: true
		});
	}

	async function joinWorkspace() {
		if (!workspaceCode) {
			toast.error('Workspace code is required');
			return;
		}
		toast.error(`Invalid workspace code: ${workspaceName}`);
	}
</script>

<Dialog.Root bind:open={workspaceDialogOpen}>
	<Dialog.Content class="m-4 max-w-[320px] p-4">
		<Tabs.Root value="create" class="max-w-[300px]">
			<Tabs.List class="bg-card-primary grid grid-cols-2">
				<Tabs.Trigger value="create">Create</Tabs.Trigger>
				<Tabs.Trigger value="join">join</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="create">
				<Label class="mb-4" for="name">Name</Label>
				<Input class="mb-4" id="name" bind:value={workspaceName} />
				<div class="flex items-center justify-between">
					<Button onclick={() => createWorkspace()}>Create Workspace</Button>
					<Button onclick={() => (workspaceDialogOpen = false)}>Cancel</Button>
				</div>
			</Tabs.Content>
			<Tabs.Content value="join">
				<Label class="mb-4" for="code">Code</Label>
				<Input class="mb-4" id="code" bind:value={workspaceCode} />
				<div class="flex items-center justify-between">
					<Button onclick={() => joinWorkspace()}>Join Workspace</Button>
					<Button onclick={() => (workspaceDialogOpen = false)}>Cancel</Button>
				</div>
			</Tabs.Content>
		</Tabs.Root>
	</Dialog.Content>
</Dialog.Root>

<Sidebar.MenuItem>
	{#if $workspaces && $workspaces.length > 0}
		<DropdownMenu.Root>
			<DropdownMenu.Trigger class="w-full">
				<Sidebar.MenuButton
					size="lg"
					class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
				>
					{#if $selectedWorkspace}
						<Avatar.Root class="h-8 w-8 rounded-lg">
							<Avatar.Image src={$selectedWorkspace.logo} alt={$selectedWorkspace.name} />
							<Avatar.Fallback class="rounded-lg">{$selectedWorkspace.name[0]}</Avatar.Fallback>
						</Avatar.Root>
					{:else}
						<Avatar.Root class="h-8 w-8 rounded-lg">
							<Avatar.Fallback class="rounded-lg">?</Avatar.Fallback>
						</Avatar.Root>
					{/if}
					<div class="grid flex-1 text-left text-sm leading-tight">
						<span class="truncate font-semibold">
							{#if $selectedWorkspace}
								{$selectedWorkspace.name}
							{:else}
								No workspace selected
							{/if}
						</span>
					</div>
					<ChevronsUpDown class="ml-auto" />
				</Sidebar.MenuButton>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content
				class="w-[--bits-dropdown-menu-anchor-width] min-w-56 rounded-lg"
				align="start"
				side={sidebar.isMobile ? 'bottom' : 'right'}
				sideOffset={4}
			>
				<DropdownMenu.Label class="text-xs text-muted-foreground">Workspaces</DropdownMenu.Label>
				{#each $workspaces as workspace (workspace.name)}
					<DropdownMenu.Item onSelect={() => {
                            $selectedWorkspace = workspace;
                            goto(`/workspace/${workspace.id}`, {
                                invalidateAll: true
                            })
                        }} class="gap-2 p-2">
						<Avatar.Root class="h-8 w-8 rounded-lg">
							<Avatar.Image src={workspace.logo} alt={workspace.name} />
							<Avatar.Fallback class="rounded-lg">{workspace.name[0]}</Avatar.Fallback>
						</Avatar.Root>
						{workspace.name}
					</DropdownMenu.Item>
				{/each}
				<DropdownMenu.Separator />
				<DropdownMenu.Item class="gap-2 p-2" onclick={() => (workspaceDialogOpen = true)}>
					<div class="flex size-6 items-center justify-center rounded-md border bg-background">
						<Plus class="size-4" />
					</div>
					<div class="font-medium text-muted-foreground">Add Workspace</div>
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	{:else if $workspaces && $workspaces.length === 0}
		<Button
			variant="ghost"
			onclick={() => (workspaceDialogOpen = true)}
			class="w-full justify-start"
		>
			<div class="grid flex-1 text-left text-sm leading-tight">
				<span class="truncate font-semibold">Create a workspace</span>
			</div>
		</Button>
	{:else}
		<div>
			<Spinner />
		</div>
	{/if}
</Sidebar.MenuItem>
