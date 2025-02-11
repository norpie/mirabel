<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { useSidebar } from '$lib/components/ui/sidebar/index.js';
	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import Plus from 'lucide-svelte/icons/plus';

	import { selectedWorkspace, workspaces } from '$lib/store';
	import Spinner from './spinner.svelte';
	import * as Avatar from '$lib/components/ui/avatar/index.js';

	let localSelectedWorkspace = $derived($selectedWorkspace);

	const sidebar = useSidebar();
</script>

<Sidebar.Menu>
	<Sidebar.MenuItem>
		{#if $workspaces && $selectedWorkspace && localSelectedWorkspace}
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
						<Sidebar.MenuButton
							{...props}
							size="lg"
							class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
						>
							<Avatar.Root class="h-8 w-8 rounded-lg">
								<Avatar.Image src={$selectedWorkspace.logo} alt={$selectedWorkspace.name} />
								<Avatar.Fallback class="rounded-lg">W</Avatar.Fallback>
							</Avatar.Root>
							<div class="grid flex-1 text-left text-sm leading-tight">
								<span class="truncate font-semibold">
									{$selectedWorkspace.name}
								</span>
							</div>
							<ChevronsUpDown class="ml-auto" />
						</Sidebar.MenuButton>
					{/snippet}
				</DropdownMenu.Trigger>
				<DropdownMenu.Content
					class="w-[--bits-dropdown-menu-anchor-width] min-w-56 rounded-lg"
					align="start"
					side={sidebar.isMobile ? 'bottom' : 'right'}
					sideOffset={4}
				>
					<DropdownMenu.Label class="text-xs text-muted-foreground">Workspaces</DropdownMenu.Label>
					{#each $workspaces as workspace (workspace.name)}
						<DropdownMenu.Item onSelect={() => selectedWorkspace.set(workspace)} class="gap-2 p-2">
							<Avatar.Root class="h-8 w-8 rounded-lg">
								<Avatar.Image src={workspace.logo} alt={workspace.name} />
								<Avatar.Fallback class="rounded-lg">W</Avatar.Fallback>
							</Avatar.Root>
							{workspace.name}
						</DropdownMenu.Item>
					{/each}
					<DropdownMenu.Separator />
					<DropdownMenu.Item class="gap-2 p-2">
						<div class="flex size-6 items-center justify-center rounded-md border bg-background">
							<Plus class="size-4" />
						</div>
						<div class="font-medium text-muted-foreground">Add workspace</div>
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{:else}
			<div>
				<Spinner />
			</div>
		{/if}
	</Sidebar.MenuItem>
</Sidebar.Menu>
