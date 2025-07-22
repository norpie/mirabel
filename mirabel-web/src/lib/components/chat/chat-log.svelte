<script lang="ts">
    import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

    let { children, messageCount, onLoadMore, isLoading = false } = $props<{
        children: any;
        messageCount: number;
        onLoadMore?: () => void;
        isLoading?: boolean;
    }>();

    let initialLoad = $state(true);
    let scrollArea: HTMLElement | null = $state(null);
    let previousMessageCount = $state(0);

    function isNearBottom(): boolean {
        if (!scrollArea) return true;
        const { scrollTop, scrollHeight, clientHeight } = scrollArea;
        return scrollTop + clientHeight >= scrollHeight - clientHeight / 2;
    }

    function isNearTop(): boolean {
        if (!scrollArea) return false;
        return scrollArea.scrollTop < 100; // Load more when within 100px of top
    }

    let lastScrollTop = $state(0);

    function handleScroll() {
        if (!scrollArea) return;
        
        // Check if we're scrolling near the top and should load more
        if (isNearTop() && !isLoading && onLoadMore) {
            onLoadMore();
        }
        
        lastScrollTop = scrollArea.scrollTop;
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
    onScrollCapture={handleScroll}
>

    {#if isLoading}
        <div class="flex justify-center p-4">
            <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-gray-900"></div>
        </div>
    {/if}
    {@render children()}
</ScrollArea>
