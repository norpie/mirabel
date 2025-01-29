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

    let repositories = $state([
        {
            name: 'Mirabel',
            logo: Github,
            platform: 'GitHub'
        },
        {
            name: 'Finanalize',
            logo: Github,
            platform: 'GitHub'
        },
        {
            name: 'Analytical',
            logo: Github,
            platform: 'GitHub'
        },
        {
            name: 'Alice',
            logo: Github,
            platform: 'GitHub'
        },
        {
            name: 'µLLM-API',
            logo: Github,
            platform: 'GitHub'
        }
    ]);
</script>

<Sidebar.Provider>
    <AppSidebar bind:user bind:repositories />
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
                            <Breadcrumb.Link href="#">Building Your Application</Breadcrumb.Link>
                        </Breadcrumb.Item>
                        <Breadcrumb.Separator class="hidden md:block" />
                        <Breadcrumb.Item>
                            <Breadcrumb.Page>Data Fetching</Breadcrumb.Page>
                        </Breadcrumb.Item>
                    </Breadcrumb.List>
                </Breadcrumb.Root>
            </div>
        </header>
    </Sidebar.Inset>
</Sidebar.Provider>
