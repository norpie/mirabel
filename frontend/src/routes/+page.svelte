<script lang="ts">
    import AppSidebar from '$lib/components/app-sidebar.svelte';
    import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
    import { Separator } from '$lib/components/ui/separator/index.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';

    import Github from 'lucide-svelte/icons/github';

    import { toast } from 'svelte-sonner';

    let user = $state({
        username: 'norpie',
        email: 'contact@norpie.dev',
        avatar: 'https://avatars.githubusercontent.com/u/46564751?v=4'
    });

    let tabName = $state('Data Fetching');

    let activeRepository = $state({
        name: 'Mirabel',
        url: 'https://github.com/norpie/mirabel',
        logo: Github,
        platform: 'GitHub'
    });

    let repositories = $state([
        {
            name: 'Mirabel',
            url: 'https://github.com/norpie/mirabel',
            logo: Github,
            platform: 'GitHub'
        },
        {
            name: 'Finanalize',
            url: 'https://github.com/norpie/finanalize',
            logo: Github,
            platform: 'GitHub'
        },
        {
            name: 'Analytical',
            url: 'https://github.com/norpie/analytical',
            logo: Github,
            platform: 'GitHub'
        },
        {
            name: 'Alice',
            url: 'https://github.com/norpie/alice',
            logo: Github,
            platform: 'GitHub'
        },
        {
            name: 'µLLM-API',
            url: 'https://github.com/norpie/uLLM-API',
            logo: Github,
            platform: 'GitHub'
        }
    ]);
</script>

<Sidebar.Provider>
    <AppSidebar bind:user bind:repositories bind:activeRepository />
    <Sidebar.Inset>
        <header
            class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
        >
            <div class="flex items-center gap-2 px-4">
                <Sidebar.Trigger class="-ml-1" />
                <Separator orientation="vertical" class="mr-2 h-4" />
                <Breadcrumb.Root>
                    <Breadcrumb.List>
                        <Breadcrumb.Item class="hidden md:block">
                            <Breadcrumb.Link href="#">{activeRepository.name}</Breadcrumb.Link>
                        </Breadcrumb.Item>
                        <Breadcrumb.Separator class="hidden md:block" />
                        <Breadcrumb.Item>
                            <Breadcrumb.Page>{tabName}</Breadcrumb.Page>
                        </Breadcrumb.Item>
                    </Breadcrumb.List>
                </Breadcrumb.Root>
            </div>
        </header>
    </Sidebar.Inset>
</Sidebar.Provider>
