<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import Mirabel from '$lib/assets/mirabel.png';
	import SendHorizontal from 'lucide-svelte/icons/send-horizontal';
	import Paperclip from 'lucide-svelte/icons/paperclip';
	import Pause from 'lucide-svelte/icons/pause';
	import type { Chat, Participant } from '$lib/models/session';
	import { selectedSession } from '$lib/store';
	import { formatTime, formatElapsedTime } from '$lib/time';
	import { SessionSocketHandler } from '$lib/socket';
	import type { SessionEvent } from '$lib/models/event';
	import { toast } from 'svelte-sonner';
	import { Separator } from '$lib/components/ui/separator';

	let {
		socket = $bindable(),
        socketStatus = $bindable(),
		chat,
        events
	}: {
		socket: SessionSocketHandler | undefined;
        socketStatus: 'open' | 'closed' | 'connecting' | 'error';
		chat: Chat | undefined;
        events: SessionEvent[] | undefined;
	} = $props();

	let socketStatusStyle = $derived(getSocketStatusStyle(socketStatus));

	let initialLoad = $state(true);

	let scrollArea: HTMLElement | null = $state(null);
	let previousMessageCount = $state(0);
	let chatInput = $state('');
    let lastChatStatus: 'sent' | 'delivered' | 'read' | 'thinking' | 'writing' | 'paused' = $derived.by(() => {
        const acknowledgementEvents = events?.filter(event => event.content.type === 'AcknowledgmentContent');
        return acknowledgementEvents?.length
            ? acknowledgementEvents[acknowledgementEvents.length - 1].content.ackType
            : null;
    });

    // Track when the current status started
    let statusStartTime = $state(new Date());
    let elapsedTime = $state('');

    function formatLastChatStatus(status: 'sent' | 'delivered' | 'read'): string {
        switch (status) {
            case 'sent':
                return 'Sent';
            case 'delivered':
                return 'Delivered';
            case 'read':
                return 'Read';
            default:
                return '';
        }
    }

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

	function getSocketStatusStyle(status: 'open' | 'closed' | 'connecting' | 'error'): {
		color: string;
		title: string;
	} {
		switch (status) {
			case 'open':
				return { color: 'bg-green-500', title: 'Connection established' };
			case 'connecting':
				return { color: 'bg-amber-500', title: 'Connecting...' };
			case 'closed':
				return { color: 'bg-red-500', title: 'Connection closed' };
			case 'error':
				return { color: 'bg-red-500', title: 'Connection error' };
			default:
				return { color: 'bg-gray-500', title: 'Unknown status' };
		}
	}

	function messageAuthor(participantId: string): Participant {
		return (
			$selectedSession?.participants.find((p) => p.id === participantId) ?? {
				id: participantId,
				name: 'Anon',
			}
		);
	}

    function isUser(participant: Participant): boolean {
        return participant.id != 'mirabel';
    }

	// Function to find the last user message
	function isLastUserMessage(index: number): boolean {
		if (!chat || chat.messages.length === 0) return false;
        if (chat.messages.length - 1 !== index) return false;
        const message = chat.messages[chat.messages.length - 1];
        if (!isUser(messageAuthor(message.authorId))) return false;
        return true;
	}

	// Helper function to determine if status should be shown as a message
	function isTypingStatus(status: string): boolean {
		return status === 'thinking' || status === 'writing' || status === 'paused';
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

	// Update status start time when status changes
	$effect(() => {
		statusStartTime = new Date();
	});

	// Update elapsed time every second
	let timer: number;

	$effect(() => {
		timer = window.setInterval(() => {
			if (isTypingStatus(lastChatStatus)) {
				elapsedTime = formatElapsedTime(statusStartTime);
			}
		}, 1000);

		return () => {
			clearInterval(timer);
		};
	});
</script>

<style>
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }

    .spinner {
        animation: spin 1.5s linear infinite;
    }
</style>

<ScrollArea
	id="chat-messages"
	class="m-2 flex h-[1px] flex-grow flex-col rounded-lg p-2 pb-0"
	bind:viewportRef={scrollArea}
>
	{#if chat}
		{#each chat.messages as msg, index}
			{@const participant = messageAuthor(msg.authorId)}
            {#if index > 0}
                <Separator class="my-4" />
            {/if}
			<div class="mb-4 flex space-x-4">
				<Avatar.Root class="h-8 w-8 rounded-lg">
					{#if msg.authorId != "mirabel"}
						<Avatar.Image src={participant.avatar} alt={`${participant.name}'s avatar`} />
						<Avatar.Fallback class="rounded-lg">{participant.name[0]}</Avatar.Fallback>
					{:else}
						<Avatar.Image src={Mirabel} alt={`${participant.name}'s avatar`} />
						<Avatar.Fallback class="rounded-lg">M</Avatar.Fallback>
					{/if}
				</Avatar.Root>
				<div class="flex flex-col">
					<div class="flex items-center gap-2">
						<p class="font-normal leading-none">{participant.name}</p>
						<p
							class="overflow-hidden whitespace-nowrap text-xs font-light leading-none text-muted-foreground"
						>
							{formatTime(msg.timestamp)}
						</p>
					</div>
					<div class="pb-2 pt-2">
						<p class="font-light">{msg.message}</p>
						{#if isLastUserMessage(index) && !isTypingStatus(lastChatStatus)}
							<p class="text-xs text-muted-foreground mt-1">{formatLastChatStatus(lastChatStatus)}</p>
						{/if}
					</div>
				</div>
			</div>
		{/each}

		<!-- Display thinking/writing/paused status as a chat message -->
		{#if isTypingStatus(lastChatStatus)}
			<div class="mb-4 flex space-x-4">
				<Avatar.Root class="h-8 w-8 rounded-lg">
					<Avatar.Image src={Mirabel} alt={`Mirabel's avatar`} />
					<Avatar.Fallback class="rounded-lg">M</Avatar.Fallback>
				</Avatar.Root>
				<div class="flex flex-col">
					<div class="flex items-center gap-2">
						<p class="font-normal leading-none">Mirabel</p>
					</div>
					<div class="pb-2 pt-2">
						<div class="flex items-center">
							{#if lastChatStatus === 'thinking'}
								<!-- Spinning animation for thinking status -->
								<div class="flex h-5 w-5 items-center justify-center">
									<svg class="spinner h-4 w-4 text-muted-foreground" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
										<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
										<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
									</svg>
								</div>
							{:else if lastChatStatus === 'writing'}
								<!-- Bouncing dots animation for writing status -->
								<div class="flex space-x-1">
									<div class="h-2 w-2 animate-bounce rounded-full bg-muted-foreground" style="animation-delay: 0ms;"></div>
									<div class="h-2 w-2 animate-bounce rounded-full bg-muted-foreground" style="animation-delay: 150ms;"></div>
									<div class="h-2 w-2 animate-bounce rounded-full bg-muted-foreground" style="animation-delay: 300ms;"></div>
								</div>
							{:else if lastChatStatus === 'paused'}
								<!-- Pause icon for paused status -->
								<div class="flex h-5 w-5 items-center justify-center">
									<Pause class="h-4 w-4 text-muted-foreground" />
								</div>
							{/if}
							<span class="ml-3 text-sm text-muted-foreground">
								{#if lastChatStatus === 'thinking'}
									Thinking for {elapsedTime}...
								{:else if lastChatStatus === 'writing'}
									Writing for {elapsedTime}...
								{:else if lastChatStatus === 'paused'}
									Paused for {elapsedTime}
								{/if}
							</span>
						</div>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</ScrollArea>

<div id="chat-input" class="relative m-2 mt-2 flex flex-row rounded-lg bg-secondary p-2">
	{#if socket}
		<div
			class="absolute left-1 top-1 h-3 w-3 rounded-full border border-secondary {socketStatusStyle.color}"
			title={socketStatusStyle.title}
		></div>
	{/if}

	<Textarea
		class="flex-1 resize-none border-none bg-transparent focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-offset-0"
		placeholder="Type your message here..."
		bind:value={chatInput}
		onkeydown={(e) => {
			if (e.key === 'Enter' && !e.shiftKey) {
				sendMessage();
                e.preventDefault();
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
