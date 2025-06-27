<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import Mirabel from '$lib/assets/mirabel.png';
	import ArrowUp from 'lucide-svelte/icons/arrow-up';
	import Plus from 'lucide-svelte/icons/plus';
	import Pause from 'lucide-svelte/icons/pause';
	import type { Chat, Participant } from '$lib/models/session';
	import { selectedSession } from '$lib/store';
	import { formatTime, formatElapsedTime } from '$lib/time';
	import type { SessionEvent } from '$lib/models/event';
	import { toast } from 'svelte-sonner';
	import { Separator } from '$lib/components/ui/separator';
	import type { User } from '$lib/models/user';
	import { getSessionState } from '$lib/session-state.svelte';
	import type { SocketHandler } from '$lib/socket.svelte';

	import Spinner from '$lib/components/throbbers/spinner.svelte';
    import Bouncer from '$lib/components/throbbers/bouncer.svelte';

	const sessionState = getSessionState();
	// Ground truth for the selected session
	let user: User | undefined = $derived(sessionState.user);
	let chat: Chat | undefined = $derived(sessionState.session?.chat ?? undefined);
	let socket: SocketHandler<SessionEvent> | undefined = $derived(sessionState.socket);

	// Status observables
	let lastChatStatus:
		| 'sent'
		| 'delivered'
		| 'seen'
		| 'thinking'
		| 'typing'
		| 'paused'
		| 'error'
		| undefined = $derived(sessionState.lastAcknowledgementType);
	let statusStartTime: Date | undefined = $derived(sessionState.lastAcknowledgementTime);
    let currentTime: Date = $state(new Date());
    let elapsedTime: string = $derived.by(() => {
        if (isTypingStatus(lastChatStatus) && statusStartTime) {
			return formatElapsedTime(statusStartTime, currentTime);
		}
        return '';
    });

	// Indicators
	let socketStatusStyle = $derived(getSocketStatusStyle(socket?.status ?? 'closed'));

	let timer: number;
	$effect(() => {
		timer = window.setInterval(() => {
            currentTime = new Date();
		}, 1000);

		return () => {
			clearInterval(timer);
		};
	});

	let scrollArea: HTMLElement | null = $state(null);
	let previousMessageCount = $state(0);

	function formatLastChatStatus(status: 'sent' | 'delivered' | 'seen' | undefined): string {
		switch (status) {
			case 'sent':
				return 'Sent';
			case 'delivered':
				return 'Delivered';
			case 'seen':
				return 'Seen';
			default:
				return '';
		}
	}

	let initialLoad = $state(true);

	function isNearBottom(): boolean {
		if (!scrollArea) return true;
		const { scrollTop, scrollHeight, clientHeight } = scrollArea;
		return scrollTop + clientHeight >= scrollHeight - clientHeight / 2;
	}

	$effect(() => {
		if (!chat || !scrollArea) return;
		const currentMessageCount = chat.messages.length;
		if (currentMessageCount > previousMessageCount || currentMessageCount > 0) {
			const shouldScroll = initialLoad || isNearBottom();
			if (shouldScroll) {
				setTimeout(() => {
					scrollArea?.scrollTo(0, scrollArea.scrollHeight);
				}, 0);
			}
			previousMessageCount = currentMessageCount;
			initialLoad = false;
		}
	});

	function getSocketStatusStyle(status: 'connecting' | 'open' | 'closing' | 'closed' | 'error'): {
		color: string;
		title: string;
	} {
		switch (status) {
			case 'open':
				return { color: 'bg-green-500', title: 'Connection established' };
			case 'connecting':
				return { color: 'bg-amber-500', title: 'Connecting...' };
			case 'closing':
				return { color: 'bg-orange-500', title: 'Closing connection...' };
			case 'closed':
				return { color: 'bg-red-500', title: 'Connection closed' };
			case 'error':
				return { color: 'bg-red-500', title: 'Connection error' };
			default:
				return { color: 'bg-gray-500', title: 'Unknown status' };
		}
	}

	function messageAuthor(participantId: string): Participant {
		if (participantId === 'mirabel') {
			return {
				id: 'mirabel',
				name: 'Mirabel',
				avatar: Mirabel
			};
		}
		return (
			$selectedSession?.participants.find((p) => p.id === participantId) ?? {
				id: participantId,
				name: 'Anon'
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
	function isTypingStatus(status: string | undefined): boolean {
		if (!status) return false;
		return status === 'thinking' || status === 'typing' || status === 'paused';
	}

	let chatInput = $state('');
	async function sendMessage() {
		if (!chatInput.trim()) return;

		if (!chat) {
			toast.error('Chat is not initialized');
			return;
		}

		if (!socket) {
			toast.error('WebSocket connection is not established');
			return;
		}

		if (!user || !user.id) {
			toast.error('User is not authenticated');
			return;
		}

		const message: SessionEvent = {
			id: 'laskjdhflasdhflk',
			source: 'user',
			timestamp: new Date().toISOString(),
			content: {
				type: 'MessageContent',
				authorId: user.id,
				message: chatInput
			}
		};

		socket.send(message);

		chatInput = '';
	}
</script>

<div id="chat" class="flex h-full flex-col">
	<ScrollArea
		id="chat-messages"
		class="m-2 h-[1px] flex flex-grow flex-col rounded-lg p-0"
		bind:viewportRef={scrollArea}
	>
		{#if chat}
			{#each chat.messages as msg, index}
				{@const participant = messageAuthor(msg.authorId)}
				{#if index > 0}
					<Separator class="mb-2 mt-4" />
				{/if}
				<div class="flex items-start space-x-2 pt-2">
					<div class="flex h-[1lh] items-center justify-center">
						<Avatar.Root
							class="flex h-8 w-8 items-center justify-center overflow-hidden rounded-lg"
						>
							<Avatar.Image
								src={participant.avatar}
								alt={`${participant.name}'s avatar`}
								class="h-full w-full object-cover"
							/>
							<Avatar.Fallback class="flex h-full w-full items-center justify-center rounded-lg">
								{participant.name[0]}
							</Avatar.Fallback>
						</Avatar.Root>
					</div>
					<div>
						<p class="[overflow-wrap:anywhere] font-normal">{msg.message}</p>
						{#if isLastUserMessage(index) && !isTypingStatus(lastChatStatus)}
							<p class="text-muted-foreground mt-1 text-xs">
								{formatLastChatStatus(lastChatStatus)}
							</p>
						{/if}
					</div>
				</div>
			{/each}

			<!-- Display thinking/typing/paused status as a chat message -->
			{#if isTypingStatus(lastChatStatus)}
				<Separator class="my-4" />
				<div class="mb-4 flex space-x-4">
					<Avatar.Root class="h-8 w-8 rounded-lg">
						<Avatar.Image src={Mirabel} alt={`Mirabel's avatar`} />
						<Avatar.Fallback class="rounded-lg">M</Avatar.Fallback>
					</Avatar.Root>
					<div class="flex flex-col justify-center">
						<div>
							<div class="flex items-center">
								{#if lastChatStatus === 'thinking'}
									<Spinner />
								{:else if lastChatStatus === 'typing'}
									<Bouncer />
								{:else if lastChatStatus === 'paused'}
									<!-- Pause icon for paused status -->
									<div class="flex h-5 w-5 items-center justify-center">
										<Pause class="text-muted-foreground h-4 w-4" />
									</div>
								{/if}
								<span class="text-muted-foreground ml-3 text-sm">
									{#if lastChatStatus === 'thinking'}
										Thinking for {elapsedTime}...
									{:else if lastChatStatus === 'typing'}
										typing for {elapsedTime}...
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

	<div
		id="chat-input"
		class="bg-secondary relative m-2 rounded-lg p-2 justify-between flex flex-col"
	>
		{#if sessionState.socket}
			<span
				class="border-secondary absolute right-1 top-1 h-3 w-3 rounded-full border {socketStatusStyle.color} z-50"
				title={socketStatusStyle.title}
			></span>
		{/if}

		<Textarea
			class="m-0 w-full rounded-lg border-none bg-transparent p-0 focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-offset-0"
			placeholder="How can I help you today?"
            bind:value={chatInput}
            maxRows={22}
            minRows={3}
            autoResize
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
			>
				<Plus class="h-5 w-5" />
			</button>
			<button
				class="hover:bg-muted/10 flex h-5 w-5 items-center justify-center rounded bg-transparent transition-colors"
				onclick={() => sendMessage()}
			>
				<ArrowUp class="h-5 w-5" />
			</button>
		</div>
	</div>
</div>
