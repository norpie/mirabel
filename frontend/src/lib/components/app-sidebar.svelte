<script lang="ts">
    import NavMain from '$lib/components/nav-main.svelte';
    import NavWorkspaces from '$lib/components/nav-workspaces.svelte';
    import NavUser from '$lib/components/nav-user.svelte';
    import TeamSwitcher from '$lib/components/team-switcher.svelte';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';

    let {
        items = $bindable(),

        ref = $bindable(null),
        collapsible = 'icon',
        ...restProps
    }: {
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

        ref?: any;
        collapsible?: 'offcanvas' | 'icon' | 'none';
    } = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
    <Sidebar.Header>
        <TeamSwitcher />
    </Sidebar.Header>
    <Sidebar.Content>
        <NavMain {items} />
        <NavWorkspaces />
    </Sidebar.Content>
    <Sidebar.Footer>
        <NavUser />
    </Sidebar.Footer>
    <Sidebar.Rail />
</Sidebar.Root>
