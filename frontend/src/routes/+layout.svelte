<script lang="ts">
    import { ModeWatcher } from 'mode-watcher';
    import { Toaster } from '$lib/components/ui/sonner/index.js';
    import '../app.css';
    let { children } = $props();

    import Github from 'lucide-svelte/icons/github';
    import Gitlab from 'lucide-svelte/icons/gitlab';
    import HardDrive from 'lucide-svelte/icons/hard-drive';
    import FileQuestion from 'lucide-svelte/icons/file-question';

    import AppSidebar from '$lib/components/app-sidebar.svelte';
    import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
    import { Separator } from '$lib/components/ui/separator/index.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';

    import BrainCircuit from 'lucide-svelte/icons/brain-circuit';

    let user = $state({
        username: 'norpie',
        email: 'contact@norpie.dev',
        avatar: 'https://avatars.githubusercontent.com/u/46564751?v=4'
    });

    function platformLogo(name: string): any {
        console.log(name);
        switch (name) {
            case 'GitHub':
                return Github;
            case 'GitLab':
                return Gitlab;
            case 'Local':
                return HardDrive;
            default:
                return FileQuestion;
        }
    }

    let repositories = $state([
        {
            name: 'Mirabel',
            url: 'https://github.com/norpie/mirabel',
            platform: 'GitHub',
            logo: Github
        },
        {
            name: 'Finanalize',
            url: 'https://github.com/norpie/finanalize',
            platform: 'GitHub',
            logo: Github
        },
        {
            name: 'Analytical',
            url: 'https://github.com/norpie/analytical',
            platform: 'GitHub',
            logo: Github
        },
        {
            name: 'Alice',
            url: 'https://github.com/norpie/alice',
            platform: 'GitHub',
            logo: Github
        },
        {
            name: 'µLLM-API',
            url: 'https://github.com/norpie/uLLM-API',
            platform: 'GitHub',
            logo: Github
        },
        {
            name: 'Random Local Project',
            platform: 'Local',
            logo: HardDrive
        }
    ]);

    // Initialize to the first repository
    let activeRepository = $state(repositories[0]);

    let chats = $state([
        {
            id: 'sdafopihv',
            title: 'Setting up the repository'
        }
    ]);

    let activeChat = $state(null);

    let items = $state([
        {
            title: 'Knowledge',
            icon: BrainCircuit,
            isActive: false,
            items: [
                {
                    id: 'technologies',
                    title: 'Technologies'
                },
                {
                    id: 'documentation',
                    title: 'Documentation'
                },
                {
                    id: 'structure',
                    title: 'Structure'
                },
                {
                    id: 'workflow',
                    title: 'Git Workflow'
                }
            ]
        }
    ]);
</script>

<ModeWatcher />
<Toaster />

<Sidebar.Provider>
    <AppSidebar bind:user bind:repositories bind:activeRepository bind:chats bind:items />
    <Sidebar.Inset>
        <header
            class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
        >
            <div class="flex items-center gap-2 px-4">
                <Sidebar.Trigger class="-ml-1" />
                <Separator orientation="vertical" class="mr-2 h-4" />
                {#if activeRepository}
                    <Breadcrumb.Root>
                        <Breadcrumb.List>
                            <Breadcrumb.Item class="hidden md:block">
                                <Breadcrumb.Link>{activeRepository.name}</Breadcrumb.Link>
                            </Breadcrumb.Item>
                            {#if activeChat}
                                <Breadcrumb.Separator class="hidden md:block" />
                                <Breadcrumb.Item>
                                    <Breadcrumb.Link>{activeChat.title}</Breadcrumb.Link>
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
