<script lang="ts">
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
    import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';
    import Ellipsis from 'lucide-svelte/icons/ellipsis';
    import Eye from 'lucide-svelte/icons/eye';
    import Star from 'lucide-svelte/icons/star';
    import Archive from 'lucide-svelte/icons/archive';

    import { sessions } from '$lib/store';
    import Spinner from './spinner.svelte';

    import { selectedWorkspace } from '$lib/store';
    import { goto } from '$app/navigation';
    import { toast } from 'svelte-sonner';
    import { del } from '$lib/request';
    import Button from './ui/button/button.svelte';

    const sidebar = useSidebar();

    async function archiveSession(sessionId: string) {
        const response = await del(`v1/workspace/${$selectedWorkspace.id}/session/${sessionId}`);
        toast.info(JSON.stringify(response));
    }
</script>

{#if $selectedWorkspace && $sessions}
    <Sidebar.Group class="group-data-[collapsible=icon]:hidden">
        <Sidebar.GroupLabel>Sessions</Sidebar.GroupLabel>
        <Sidebar.Menu>
            {#if $sessions}
                {#each $sessions as session (session.title)}
                    <Sidebar.MenuItem>
                        <Sidebar.MenuButton>
                            {#snippet child({ props })}
                                <a
                                    {...props}
                                    href={`/workspace/${$selectedWorkspace.id}/session/${session.id}`}
                                >
                                    <span class="block w-full truncate">{session.title}</span>
                                </a>
                            {/snippet}
                        </Sidebar.MenuButton>
                        <DropdownMenu.Root>
                            <DropdownMenu.Trigger>
                                {#snippet child({ props })}
                                    <Sidebar.MenuAction showOnHover {...props}>
                                        <Ellipsis />
                                    </Sidebar.MenuAction>
                                {/snippet}
                            </DropdownMenu.Trigger>
                            <DropdownMenu.Content
                                class="w-48 rounded-lg"
                                side={sidebar.isMobile ? 'bottom' : 'right'}
                                align={sidebar.isMobile ? 'end' : 'start'}
                            >
                                <DropdownMenu.Item>
                                    <Eye class="text-muted-foreground" />
                                    <a
                                        href={`/workspace/${$selectedWorkspace.id}/session/${session.id}`}
                                        >View Session</a
                                    >
                                </DropdownMenu.Item>
                                <DropdownMenu.Item>
                                    <Star class="text-muted-foreground" />
                                    <Button
                                        size="pseudoATag"
                                        variant="ghost"
                                        onclick={() => toast.info('Not implemented yet.')}
                                        >Favourite Session</Button
                                    >
                                </DropdownMenu.Item>
                                <DropdownMenu.Separator />
                                <DropdownMenu.Item>
                                    <Archive class="text-muted-foreground" />
                                    <Button
                                        size="pseudoATag"
                                        variant="ghost"
                                        onclick={() => archiveSession(session.id)}
                                        >Archive Session</Button
                                    >
                                </DropdownMenu.Item>
                            </DropdownMenu.Content>
                        </DropdownMenu.Root>
                    </Sidebar.MenuItem>
                {/each}
            {:else}
                <Sidebar.MenuItem>
                    <Spinner />
                </Sidebar.MenuItem>
            {/if}
        </Sidebar.Menu>
    </Sidebar.Group>
{/if}
