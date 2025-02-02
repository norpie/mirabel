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
    import { fetchAllProjects } from '$lib/api/project';
    import { fetchAllChats } from '$lib/api/chat';

    import { user, projects, selectedProject, chats, selectedChat, ready } from '$lib/store';

    onMount(async () => {
        user.set(await fetchUser());
        projects.set((await fetchAllProjects({ page: 1, pageSize: 10 })).data);
        selectedProject.set($projects[0]);
        chats.set((await fetchAllChats($selectedProject.id, { page: 1, pageSize: 10 })).data);
        selectedChat.set($chats[0]);
    });

    $inspect({ $user, $projects, $selectedProject, $chats, $selectedChat, $ready });

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

{#if $ready}
    <Sidebar.Provider>
        <AppSidebar bind:items />
        <Sidebar.Inset>
            <header
                class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
            >
                <div class="flex items-center gap-2 px-4">
                    <Sidebar.Trigger class="-ml-1" />
                    <Separator orientation="vertical" class="mr-2 h-4" />
                    {#if selectedProject}
                        <Breadcrumb.Root>
                            <Breadcrumb.List>
                                <Breadcrumb.Item class="hidden md:block">
                                    <Breadcrumb.Link>{selectedProject.name}</Breadcrumb.Link>
                                </Breadcrumb.Item>
                                {#if selectedChat}
                                    <Breadcrumb.Separator class="hidden md:block" />
                                    <Breadcrumb.Item>
                                        <Breadcrumb.Link>{selectedChat.title}</Breadcrumb.Link>
                                    </Breadcrumb.Item>
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
