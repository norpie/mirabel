<script lang="ts">
    import * as Avatar from '$lib/components/ui/avatar/index.js';
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';
    import * as Dialog from '$lib/components/ui/dialog/index.js';
    import * as ScrollArea from '$lib/components/ui/scroll-area/index.js';

    import { useSidebar } from '$lib/components/ui/sidebar/index.js';

    import { setMode, mode } from 'mode-watcher';

    import Spinner from '$lib/components/spinner.svelte';

    import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
    import LogOut from 'lucide-svelte/icons/log-out';
    import Sun from 'lucide-svelte/icons/sun';
    import Moon from 'lucide-svelte/icons/moon';
    import Settings from 'lucide-svelte/icons/settings';

    import SettingsInsert from '$lib/components/settings.svelte';

    import { user } from '$lib/store';
    import { authStore } from '$lib/auth/store.svelte.js';
    import { goto } from '$app/navigation';

    const sidebar = useSidebar();

    async function logout() {
        await authStore.logout();
    }

    let settingsOpen = $state(false);
</script>

<Dialog.Root bind:open={settingsOpen}>
    <Dialog.Content class="flex h-[85vh] min-w-[50vw] max-w-[75vw] flex-col overflow-hidden">
        <ScrollArea.Root class="flex-1">
            <SettingsInsert />
        </ScrollArea.Root>
    </Dialog.Content>
</Dialog.Root>

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
                                <Avatar.Fallback class="rounded-lg"
                                    >{$user.username[0]}</Avatar.Fallback
                                >
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
                                <Avatar.Fallback class="rounded-lg"
                                    >{$user.username[0]}</Avatar.Fallback
                                >
                            </Avatar.Root>
                            <div class="grid flex-1 text-left text-sm leading-tight">
                                <span class="truncate font-semibold">{$user.username}</span>
                                <span class="truncate text-xs">{$user.email}</span>
                            </div>
                        </div>
                    </DropdownMenu.Label>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item onclick={() => (settingsOpen = true)}>
                        <Settings />
                        Settings
                    </DropdownMenu.Item>
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
