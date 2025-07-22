<script lang="ts">
    import * as Chat from '$lib/components/chat/index';
    import Mirabel from '$lib/assets/mirabel.png';
    import type { TimelineEntry, TimelineMessage, UserInteraction } from '$lib/models/session';
    import { toast } from 'svelte-sonner';
    import { Separator } from '$lib/components/ui/separator';
    import type { User } from '$lib/models/user';
    import { SessionState } from '$lib/session-state.svelte';
    import type { SocketHandler } from '$lib/socket.svelte';

    let {
        sessionState = $bindable()
    }: {
        sessionState: SessionState;
    } = $props();

    // Ground truth for the selected session
    let user: User = $derived(sessionState.user);
    let timeline: TimelineEntry[] = $derived(sessionState.timeline);
    let timelineCount: number = $derived(sessionState.timelineKnownTotal);

    let messages: TimelineMessage[] = $derived.by(() => {
        return timeline.filter((entry) => entry.contentType === 'message');
    });

    let lastUserMessage: TimelineMessage | undefined = $derived.by(() => {
        for (let i = messages.length - 1; i >= 0; i--) {
            if (messages[i].content.sender === 'user') {
                return messages[i];
            }
        }
        return undefined;
    });

    let socket: SocketHandler<TimelineEntry, UserInteraction> | undefined = $derived(
        sessionState?.socket
    );

    function messageAuthor(sender: string): {
        id: string;
        name: string;
        avatar?: any;
    } {
        if (sender === 'agent') {
            return {
                id: 'mirabel',
                name: 'Mirabel',
                avatar: Mirabel
            };
        } else {
            return {
                id: user.id,
                name: user.username,
                avatar: user.avatar
            };
        }
    }

    let chatInput = $state('');

    function handleReconnect() {
        if (socket) {
            socket.manualReconnect();
        }
    }

    async function sendMessage() {
        if (!socket) {
            toast.error('Socket is not connected. Please try again later.');
            return;
        }
        socket.send({
            type: 'message',
            content: chatInput
        });
        chatInput = '';
    }

    async function handleLoadMore() {
        await sessionState.loadOlderMessages();
    }
</script>

<Chat.Root>
    <Chat.Log 
        messageCount={timelineCount}
        onLoadMore={handleLoadMore}
        isLoading={sessionState.isLoadingOlder}
    >
        {#each messages as msg, index}
            {#if index > 0}
                <Separator class="mb-2 mt-4" />
            {/if}
            {#if lastUserMessage && msg.id === lastUserMessage.id}
                <Chat.Message
                    acknowledgment={sessionState.lastAcknowledgementType}
                    message={msg.content.message}
                    author={messageAuthor(msg.content.sender)}
                />
            {:else}
                <Chat.Message
                    message={msg.content.message}
                    author={messageAuthor(msg.content.sender)}
                />
            {/if}
        {/each}
        {#if sessionState.agentStatus && sessionState.agentStatusTime}
            <Separator class="my-4" />
            <Chat.ChatTypingIndicator
                mirabelStatus={sessionState.agentStatus}
                lastAcknowledgementTime={sessionState.agentStatusTime}
            />
        {/if}
    </Chat.Log>
    <Chat.Input
        socketStatus={sessionState.socket?.status}
        bind:value={chatInput}
        send={sendMessage}
        onReconnect={handleReconnect}
        disabled={!timeline}
    />
</Chat.Root>
