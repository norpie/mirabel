<script lang="ts">
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';
    import { cn } from '$lib/utils';

    let {
        ref = $bindable(null),
        class: className = '',
        collapsible = 'icon',
        items = $bindable([]),
        activeTab = $bindable('profile'),
        ...restProps
    }: {
        ref?: any;
        class?: string;
        collapsible?: 'offcanvas' | 'icon' | 'none';
        items: { title: string; items: { title: string; value: string; component?: string }[] }[];
        activeTab: string;
    } = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps} class={className}>
    <Sidebar.Content>
        {#each items as group (group.title)}
            <Sidebar.Group>
                <Sidebar.GroupLabel>{group.title}</Sidebar.GroupLabel>
                <Sidebar.GroupContent>
                    <Sidebar.Menu>
                        {#each group.items as item (item.title)}
                            {@const activeClass = activeTab === item.value ? 'bg-muted' : ''}
                            <Sidebar.MenuItem class={cn('rounded-md', activeClass)}>
                                <Sidebar.MenuButton
                                    onclick={() => {
                                        activeTab = item.value;
                                    }}
                                >
                                    <p>{item.title}</p>
                                </Sidebar.MenuButton>
                            </Sidebar.MenuItem>
                        {/each}
                    </Sidebar.Menu>
                </Sidebar.GroupContent>
            </Sidebar.Group>
        {/each}
    </Sidebar.Content>
    <!-- <Sidebar.Rail /> -->
</Sidebar.Root>
