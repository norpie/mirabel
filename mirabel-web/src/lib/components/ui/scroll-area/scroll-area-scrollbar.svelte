<script lang="ts">
    import { ScrollArea as ScrollAreaPrimitive, type WithoutChild } from 'bits-ui';
    import { cn } from '$lib/utils.js';

    let {
        ref = $bindable(null),
        class: className,
        thumbClass = '',
        orientation = 'vertical',
        children,
        ...restProps
    }: WithoutChild<ScrollAreaPrimitive.ScrollbarProps> = $props();
</script>

<ScrollAreaPrimitive.Scrollbar
    bind:ref
    {orientation}
    class={cn(
        'flex touch-none select-none transition-colors',
        orientation === 'vertical' && 'h-full w-2.5 border-l border-l-transparent p-px',
        orientation === 'horizontal' && 'h-2.5 w-full border-t border-t-transparent p-px',
        className
    )}
    {...restProps}
>
    {@render children?.()}
    <ScrollAreaPrimitive.Thumb
        class={cn(
            'relative rounded-full bg-border',
            thumbClass,
            orientation === 'vertical' && 'flex-1'
        )}
    />
</ScrollAreaPrimitive.Scrollbar>
