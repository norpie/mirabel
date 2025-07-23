<script lang="ts">
    import Plus from 'lucide-svelte/icons/plus';
    import { Textarea } from '../ui/textarea';
    import ArrowUp from 'lucide-svelte/icons/arrow-up';
    import { Button } from '../ui/button';

    let {
        value = $bindable(''),
        send,
        socketStatus,
        onReconnect,
        disabled = false
    }: {
        value?: string;
        send: () => void;
        socketStatus?: 'connecting' | 'open' | 'closing' | 'closed' | 'error';
        onReconnect?: () => void;
        disabled?: boolean;
    } = $props();

    let color = $derived.by(() => {
        if (socketStatus) {
            return colorFromSocketStatus(socketStatus).color;
        }
        return 'bg-gray-500';
    });
    let title = $derived.by(() => {
        if (socketStatus) {
            return titleFromSocketStatus(socketStatus);
        }
        return 'Unknown';
    });

    function colorFromSocketStatus(
        status: 'connecting' | 'open' | 'closing' | 'closed' | 'error'
    ): { color: string; title: string } {
        switch (status) {
            case 'open':
                return { color: 'bg-green-500 animate-pulse', title: 'Connection established' };
            case 'connecting':
                return { color: 'bg-amber-500 animate-pulse', title: 'Connecting...' };
            case 'closing':
                return { color: 'bg-orange-500 animate-pulse', title: 'Closing connection...' };
            case 'closed':
                return {
                    color: 'bg-red-500 cursor-pointer hover:bg-red-400',
                    title: 'Connection closed - Click to reconnect'
                };
            case 'error':
                return {
                    color: 'bg-red-500 cursor-pointer hover:bg-red-400',
                    title: 'Connection error - Click to reconnect'
                };
            default:
                return { color: 'bg-gray-500', title: 'Unknown status' };
        }
    }

    function titleFromSocketStatus(
        status: 'connecting' | 'open' | 'closing' | 'closed' | 'error'
    ): string {
        switch (status) {
            case 'open':
                return 'Connection established';
            case 'connecting':
                return 'Connecting...';
            case 'closing':
                return 'Closing connection...';
            case 'closed':
                return 'Connection closed - Click to reconnect';
            case 'error':
                return 'Connection error - Click to reconnect';
            default:
                return 'Unknown status';
        }
    }

    function handleStatusClick() {
        if ((socketStatus === 'closed' || socketStatus === 'error') && onReconnect) {
            onReconnect();
        }
    }

    function sendMessage() {
        if (!value.trim()) {
            return;
        }
        send();
    }
</script>

<div id="chat-input" class="relative m-2 flex flex-col justify-between rounded-lg bg-secondary p-2">
    <Button
        variant="indicator"
        size="indicator"
        class="absolute right-1 top-1 {color} z-50"
        {title}
        onclick={handleStatusClick}
        disabled={!(socketStatus === 'closed' || socketStatus === 'error')}
    ></Button>

    <Textarea
        class="m-0 w-full rounded-lg border-none bg-transparent p-0 focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-offset-0"
        placeholder="How can I help you today?"
        bind:value
        maxRows={22}
        minRows={3}
        autoResize
        {disabled}
        onkeydown={(e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                sendMessage();
                e.preventDefault();
            }
        }}
    />
    <div class="flex items-center justify-between pt-2">
        <button
            class="flex h-5 w-5 items-center justify-center rounded bg-transparent transition-colors hover:bg-muted/10"
            {disabled}
        >
            <Plus class="h-5 w-5" />
        </button>
        <button
            class="flex h-5 w-5 items-center justify-center rounded bg-transparent transition-colors hover:bg-muted/10"
            onclick={() => sendMessage()}
            {disabled}
        >
            <ArrowUp class="h-5 w-5" />
        </button>
    </div>
</div>
