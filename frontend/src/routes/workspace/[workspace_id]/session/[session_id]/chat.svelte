<script lang="ts">
	import * as Chat from '$lib/components/chat/index';
	import Mirabel from '$lib/assets/mirabel.png';
	import type { Chat as ChatModel, Participant } from '$lib/models/session';
	import { selectedSession } from '$lib/store';
	import type { SessionEvent } from '$lib/models/event';
	import { toast } from 'svelte-sonner';
	import { Separator } from '$lib/components/ui/separator';
	import type { User } from '$lib/models/user';
	import { getSessionState } from '$lib/session-state.svelte';
	import type { SocketHandler } from '$lib/socket.svelte';

	const sessionState = getSessionState();
	// Ground truth for the selected session
	let user: User = $derived(sessionState.user);
	let chat: ChatModel = $derived(sessionState.session?.chat ?? undefined);
	let messageCount: number = $derived(chat?.messages.length ?? 0);
	let socket: SocketHandler<SessionEvent> | undefined = $derived(sessionState.socket);
	let lastAcknowledgementTime: Date = $derived(sessionState.lastAcknowledgementTime ?? new Date());

	let log: Chat.Log | undefined = $state(undefined);

	let mirabelStatus: 'thinking' | 'typing' | 'paused' | undefined = $derived.by(() => {
		switch (sessionState.lastAcknowledgementType) {
			case 'thinking':
			case 'typing':
			case 'paused':
				return sessionState.lastAcknowledgementType;
			default:
				return undefined;
		}
	});

	let sentStatus: 'sent' | 'delivered' | 'seen' | undefined = $derived.by(() => {
		switch (sessionState.lastAcknowledgementType) {
			case 'sent':
			case 'delivered':
			case 'seen':
				return sessionState.lastAcknowledgementType;
			default:
				return undefined;
		}
	});

	function messageAuthor(participantId: string): Participant {
		if (participantId === 'mirabel') {
			return {
				id: 'mirabel',
				name: 'Mirabel',
				avatar: Mirabel
			};
		}
		return (
			sessionState.session.participants.find((p) => p.id === participantId) ?? {
				id: participantId,
				name: 'Anon'
			}
		);
	}

	let chatInput = $state('');

	async function sendMessage() {
		if (!socket) {
			toast.error('Socket is not connected. Please try again later.');
			return;
		}
		socket.send({
			id: crypto.randomUUID(),
			source: 'user',
			timestamp: new Date().toISOString(),
			content: {
				type: 'MessageContent',
				authorId: user.id,
				message: chatInput
			}
		});
		chatInput = '';
	}
</script>

<Chat.Root>
	<Chat.Log {messageCount}>
		{#each chat.messages as msg, index}
			{#if index > 0}
				<Separator class="mb-2 mt-4" />
			{/if}
			{#if index === chat.messages.length - 1 && msg.authorId !== 'mirabel'}
				<Chat.Message
					acknowledgment={sentStatus}
					message={msg.message}
					author={messageAuthor(msg.authorId)}
				/>
			{:else}
				<Chat.Message message={msg.message} author={messageAuthor(msg.authorId)} />
			{/if}
		{/each}
		{#if mirabelStatus}
			<Separator class="my-4" />
			<Chat.ChatTypingIndicator {mirabelStatus} {lastAcknowledgementTime} />
		{/if}
	</Chat.Log>
	<Chat.Input
		socketStatus={sessionState.socket?.status}
		bind:value={chatInput}
		send={sendMessage}
		disabled={!chat}
	/>
</Chat.Root>
