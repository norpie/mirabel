<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { useSidebar } from '$lib/components/ui/sidebar/index.js';

	import { setMode, mode } from 'mode-watcher';

	import Spinner from '$lib/components/spinner.svelte';

	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import LogOut from 'lucide-svelte/icons/log-out';
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';

	import { user } from '$lib/store';
	import { del } from '$lib/request';
	import { goto } from '$app/navigation';

	const sidebar = useSidebar();

    async function logout() {
        await del("v1/auth/logout");
        localStorage.removeItem('accessToken');
        goto('/login');
    }
</script>

<Sidebar.Menu>
	{#if $user}
		<Sidebar.MenuItem>
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
						<Sidebar.MenuButton
							size="lg"
							class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
							{...props}
						>
							<Avatar.Root class="h-8 w-8 rounded-lg">
								<Avatar.Image src={$user.avatar} alt={$user.username} />
								<Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
							</Avatar.Root>
							<div class="grid flex-1 text-left text-sm leading-tight">
								<span class="truncate font-semibold">{$user.username}</span>
								<span class="truncate text-xs">{$user.email}</span>
							</div>
							<ChevronsUpDown class="ml-auto size-4" />
						</Sidebar.MenuButton>
					{/snippet}
				</DropdownMenu.Trigger>
				<DropdownMenu.Content
					class="w-[--bits-dropdown-menu-anchor-width] min-w-56 rounded-lg"
					side={sidebar.isMobile ? 'bottom' : 'right'}
					align="end"
					sideOffset={4}
				>
					<DropdownMenu.Label class="p-0 font-normal">
						<div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
							<Avatar.Root class="h-8 w-8 rounded-lg">
								<Avatar.Image src={$user.avatar} alt={$user.username} />
								<Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
							</Avatar.Root>
							<div class="grid flex-1 text-left text-sm leading-tight">
								<span class="truncate font-semibold">{$user.username}</span>
								<span class="truncate text-xs">{$user.email}</span>
							</div>
						</div>
					</DropdownMenu.Label>
					<DropdownMenu.Separator />
					{#if $mode == 'dark'}
						<DropdownMenu.Item onclick={() => setMode('light')}>
							<Sun />
							Light mode
						</DropdownMenu.Item>
					{:else}
						<DropdownMenu.Item onclick={() => setMode('dark')}>
							<Moon />
							Dark mode
						</DropdownMenu.Item>
					{/if}
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={() => logout()}>
						<LogOut />
						Log out
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</Sidebar.MenuItem>
	{:else}
		<div class="flex h-[100%] w-full items-center justify-center">
			<Spinner />
		</div>
	{/if}
</Sidebar.Menu>
