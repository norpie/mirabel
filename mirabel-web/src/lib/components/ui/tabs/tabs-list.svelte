<script lang="ts">
    import { Tabs as TabsPrimitive } from 'bits-ui';
    import { cn } from '$lib/utils.js';

    let {
        ref = $bindable(null),
        class: className,
        ...restProps
    }: TabsPrimitive.ListProps = $props();
</script>

<TabsPrimitive.List
    bind:ref
    class={cn('inline-flex items-center justify-center gap-2 rounded-md bg-muted', className)}
    {...restProps}
/>
