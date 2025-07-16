<script lang="ts">
	import Plus from 'lucide-svelte/icons/plus';
	import { Textarea } from '../ui/textarea';
	import ArrowUp from 'lucide-svelte/icons/arrow-up';

	let {
		value = $bindable(''),
		send,
		socketStatus,
		onReconnect,
		disabled = false
	}: {
		value?: string;
		send: () => void;
		socketStatus?: 'connecting' | 'open' | 'closing' | 'closed' | 'error',
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

	function colorFromSocketStatus(status: 'connecting' | 'open' | 'closing' | 'closed' | 'error'): { color: string; title: string } {
		switch (status) {
			case 'open':
				return { color: 'bg-green-500 animate-pulse', title: 'Connection established' };
			case 'connecting':
				return { color: 'bg-amber-500 animate-pulse', title: 'Connecting...' };
			case 'closing':
				return { color: 'bg-orange-500 animate-pulse', title: 'Closing connection...' };
			case 'closed':
				return { color: 'bg-red-500 cursor-pointer hover:bg-red-400', title: 'Connection closed - Click to reconnect' };
			case 'error':
				return { color: 'bg-red-500 cursor-pointer hover:bg-red-400', title: 'Connection error - Click to reconnect' };
			default:
				return { color: 'bg-gray-500', title: 'Unknown status' };
		}
	}

	function titleFromSocketStatus(status: 'connecting' | 'open' | 'closing' | 'closed' | 'error'): string {
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

<div id="chat-input" class="bg-secondary relative m-2 flex flex-col justify-between rounded-lg p-2">
	<span
		class="border-secondary absolute right-1 top-1 h-3 w-3 rounded-full border {color} z-50"
		{title}
		onclick={handleStatusClick}
		role={socketStatus === 'closed' || socketStatus === 'error' ? 'button' : undefined}
		tabindex={socketStatus === 'closed' || socketStatus === 'error' ? 0 : undefined}
	></span>

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
			class="hover:bg-muted/10 flex h-5 w-5 items-center justify-center rounded bg-transparent transition-colors"
			{disabled}
			>
			<Plus class="h-5 w-5" />
		</button>
		<button
			class="hover:bg-muted/10 flex h-5 w-5 items-center justify-center rounded bg-transparent transition-colors"
			onclick={() => sendMessage()}
			{disabled}
		>
			<ArrowUp class="h-5 w-5" />
		</button>
	</div>
</div>
