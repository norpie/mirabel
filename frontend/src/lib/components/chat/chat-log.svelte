<script lang="ts">
    import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

    let { children, messageCount } = $props();

    let initialLoad = $state(true);
    let scrollArea: HTMLElement | null = $state(null);
    let previousMessageCount = $state(0);

    function isNearBottom(): boolean {
        if (!scrollArea) return true;
        const { scrollTop, scrollHeight, clientHeight } = scrollArea;
        return scrollTop + clientHeight >= scrollHeight - clientHeight / 2;
    }

    $effect(() => {
        if (messageCount > previousMessageCount || messageCount > 0) {
            const shouldScroll = initialLoad || isNearBottom();
            if (shouldScroll) {
                setTimeout(() => {
                    scrollArea?.scrollTo(0, scrollArea.scrollHeight);
                }, 0);
            }
            previousMessageCount = messageCount;
            initialLoad = false;
        }
    });
</script>

<ScrollArea
    id="chat-messages"
    class="m-2 flex h-[1px] flex-grow flex-col rounded-lg p-0"
    bind:viewportRef={scrollArea}
>
    {@render children()}
</ScrollArea>
