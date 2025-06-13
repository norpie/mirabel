<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import Mirabel from '$lib/assets/mirabel.png';
	import SendHorizontal from 'lucide-svelte/icons/send-horizontal';
	import Paperclip from 'lucide-svelte/icons/paperclip';
	import type { Chat, Participant } from '$lib/models/session';
	import { selectedSession } from '$lib/store';
	import { SessionSocketHandler } from '$lib/socket';
	import type { SessionEvent } from '$lib/models/event';
	import { toast } from 'svelte-sonner';

	let { socket = $bindable(), chat }: {
        socket: SessionSocketHandler | undefined,
        chat: Chat | undefined
    } = $props();

    let scrollArea: HTMLElement | null = $state(null);
    let previousMessageCount = $state(0);
    let initialLoad = $state(true);
    
    // Function to check if we're near the bottom of the scroll area
    function isNearBottom(): boolean {
        if (!scrollArea) return true;
        
        const { scrollTop, scrollHeight, clientHeight } = scrollArea;
        // If we're within half a viewport of the bottom
        return scrollTop + clientHeight >= scrollHeight - clientHeight / 2;
    }
    
    // Effect to handle auto-scrolling when new messages arrive
    $effect(() => {
        if (!chat || !scrollArea) return;
        
        const currentMessageCount = chat.messages.length;
        
        // If we have new messages
        if (currentMessageCount > previousMessageCount || currentMessageCount > 0) {
            // Always scroll on initial load, otherwise check position
            const shouldScroll = initialLoad || isNearBottom();
            
            if (shouldScroll) {
                // Use setTimeout to ensure DOM is updated before scrolling
                setTimeout(() => {
                    scrollArea?.scrollTo(0, scrollArea.scrollHeight);
                }, 0);
            }
            
            // Update the message count and mark initial load as complete
            previousMessageCount = currentMessageCount;
            initialLoad = false;
        }
    });

	let chatInput = $state('');

	function formatTime(iso8601: string): string {
		const date = new Date(iso8601);
		const now = new Date();
		const sameDay = date.toDateString() === now.toDateString();

		const hours = String(date.getHours()).padStart(2, '0');
		const minutes = String(date.getMinutes()).padStart(2, '0');

		if (!sameDay) {
			const day = date.getDate();
			const month = date.toLocaleString('default', { month: 'short' });
			const year = date.getFullYear();

			if (year !== now.getFullYear()) {
				return `${month} ${day}, ${year}, ${hours}:${minutes}`;
			}

			return `${month} ${day}, ${hours}:${minutes}`;
		}

		return `${hours}:${minutes}`;
	}

	function messageAuthor(participantId: string): Participant {
		return (
			$selectedSession?.participants.find((p) => p.id === participantId) ?? {
				id: 'unknown',
				name: 'Anon',
				user: true
			}
		);
	}

	async function sendMessage() {
		if (!chatInput.trim()) return;

		if (!socket) {
			toast.error('WebSocket connection is not established');
			return;
		}

		const message: SessionEvent = {
			id: 'laskjdhflasdhflk',
			source: 'user',
			timestamp: new Date().toISOString(),
			content: {
				type: 'MessageContent',
				message: chatInput
			}
		};
		console.log('Sending message:', message);

		socket.send(message);

		chatInput = '';
	}
</script>

<ScrollArea
	id="chat-messages"
	class="m-2 flex h-[1px] flex-grow flex-col rounded-lg p-2 pb-0"
    bind:viewportRef={scrollArea}
>
    {#if chat}
	{#each chat.messages as msg}
	{@const participant = messageAuthor(msg.participant)}
		<div class="mb-4 flex space-x-4">
			<Avatar.Root class="h-8 w-8 rounded-lg">
				{#if participant.user}
					<Avatar.Image src={participant.avatar} alt={`${participant.name}'s avatar`} />
					<Avatar.Fallback class="rounded-lg"></Avatar.Fallback>
				{:else}
					<Avatar.Image src={Mirabel} alt={`${participant.name}'s avatar`} />
					<Avatar.Fallback class="rounded-lg">M</Avatar.Fallback>
				{/if}
			</Avatar.Root>
			<div class="flex flex-col">
				<div class="flex items-center gap-2">
					<p class="font-normal leading-none">{participant.name}</p>
					<p class="text-xs font-light leading-none text-muted-foreground overflow-hidden whitespace-nowrap">
						{formatTime(msg.timestamp)}
					</p>
				</div>
				<div class="pb-2 pt-2">
					<p class="font-light">{msg.message}</p>
				</div>
			</div>
		</div>
	{/each}
    {/if}
</ScrollArea>

<div id="chat-input" class="m-2 mt-2 flex flex-row rounded-lg bg-secondary p-2">
	<Textarea
		class="flex-1 resize-none border-none bg-transparent focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-offset-0"
		placeholder="Type your message here..."
		bind:value={chatInput}
		onkeydown={(e) => {
			if (e.key === 'Enter' && !e.shiftKey) {
				sendMessage();
			}
		}}
	/>
	<div id="buttons" class="flex flex-col gap-1 pl-2">
		<Button onclick={() => sendMessage()}>
			<SendHorizontal class="pointer-events-none" />
		</Button>
		<Button>
			<Paperclip />
		</Button>
	</div>
</div>
