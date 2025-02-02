<script lang="ts">
    import NavMain from '$lib/components/nav-main.svelte';
    import NavProjects from '$lib/components/nav-projects.svelte';
    import NavUser from '$lib/components/nav-user.svelte';
    import TeamSwitcher from '$lib/components/team-switcher.svelte';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';

    let {
        user = $bindable(),
        repositories = $bindable(),
        activeRepository = $bindable(),
        chats = $bindable(),
        items = $bindable(),

        ref = $bindable(null),
        collapsible = 'icon',
        ...restProps
    }: {
        user: { username: string; email: string; avatar: string };
        repositories: { name: string; logo: any; platform: string }[];
        activeRepository: { name: string; logo: any; platform: string };
        chats: { id: string; title: string }[];
        items: {
            title: string;
            url: string;
            icon: any;
            isActive: boolean;
            items: {
                title: string;
                url: string;
            }[];
        }[];

        ref: any;
        collapsible?: 'offcanvas' | 'icon' | 'none';
    } = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
    <Sidebar.Header>
        <TeamSwitcher bind:repositories bind:activeRepository />
    </Sidebar.Header>
    <Sidebar.Content>
        <NavMain {items} />
        <NavProjects bind:chats />
    </Sidebar.Content>
    <Sidebar.Footer>
        <NavUser {user} />
    </Sidebar.Footer>
    <Sidebar.Rail />
</Sidebar.Root>
