<script lang="ts">
    import type { WithElementRef, WithoutChildren } from 'bits-ui';
    import type { HTMLTextareaAttributes } from 'svelte/elements';
    import { cn } from '$lib/utils.js';

    let {
        ref = $bindable(null),
        value = $bindable(),
        maxRows = 5,
        minRows = 5,
        autoResize = false,
        class: className,
        ...restProps
    }: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> & {
        maxRows?: number;
        minRows?: number;
        autoResize?: boolean;
    } = $props();

    // Auto-resize calculations
    let minHeight = $derived(`${1 + minRows * 1.2}em`);
    let maxHeight = $derived(maxRows ? `${1 + maxRows * 1.2}em` : 'auto');

    // Base textarea classes
    const baseClasses =
        'border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[80px] w-full rounded-md border px-3 py-2 text-base focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm';
</script>

{#if autoResize}
    <div class="relative w-full">
        <pre
            aria-hidden="true"
            class={cn(
                baseClasses,
                'pointer-events-none invisible whitespace-pre-wrap break-words',
                className
            )}
            style="min-height: {minHeight}; max-height: {maxHeight}; overflow: hidden;">{(value ||
                '') + '\n'}</pre>

        <textarea
            bind:this={ref}
            class={cn(
                baseClasses,
                'absolute inset-0 h-full resize-none overflow-y-auto overflow-x-hidden',
                className
            )}
            bind:value
            style="min-height: {minHeight}; max-height: {maxHeight};"
            {...restProps}
        ></textarea>
    </div>
{:else}
    <textarea bind:this={ref} class={cn(baseClasses, className)} bind:value {...restProps}
    ></textarea>
{/if}
