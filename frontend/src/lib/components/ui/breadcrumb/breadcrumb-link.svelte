<script lang="ts">
    import type { HTMLHtmlAttributes } from 'svelte/elements';
    import type { Snippet } from 'svelte';
    import type { WithElementRef } from 'bits-ui';
    import { cn } from '$lib/utils.js';

    let {
        ref = $bindable(null),
        class: className,
        child,
        children,
        ...restProps
    }: WithElementRef<HTMLHtmlAttributes> & {
        child?: Snippet<[{ props: HTMLHtmlAttributes }]>;
    } = $props();

    const attrs = $derived({
        class: cn('text-foreground', className),
        ...restProps
    });
</script>

{#if child}
    {@render child({ props: attrs })}
{:else}
    <p bind:this={ref} {...attrs}>
        {@render children?.()}
    </p>
{/if}
