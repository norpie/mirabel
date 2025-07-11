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

	let socket: SocketHandler<TimelineEntry, UserInteraction> | undefined = $derived(
		sessionState.socket
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
</script>

<Chat.Root>
	<Chat.Log messageCount={timelineCount}>
		{#each messages as msg, index}
			{#if index > 0}
				<Separator class="mb-2 mt-4" />
			{/if}
			{#if index === timeline.length - 1 && msg.content.sender !== 'agent'}
				<Chat.Message
					acknowledgment={sessionState.lastAcknowledgementType}
					message={msg.content.message}
					author={messageAuthor(msg.content.sender)}
				/>
			{:else}
				<Chat.Message message={msg.content.message} author={messageAuthor(msg.content.sender)} />
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
		disabled={!timeline}
	/>
</Chat.Root>
