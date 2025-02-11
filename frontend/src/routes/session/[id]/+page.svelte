<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';

	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	import Mirabel from '$lib/assets/mirabel.png';

	import type { PaneAPI } from 'paneforge';

	import { selectedSession, breadcrumbs } from '$lib/store';

	import ChevronsLeft from 'lucide-svelte/icons/chevrons-left';
	import ChevronsRight from 'lucide-svelte/icons/chevrons-right';
	import Spinner from '$lib/components/spinner.svelte';
	import SendHorizontal from 'lucide-svelte/icons/send-horizontal';
	import Paperclip from 'lucide-svelte/icons/paperclip';

	let sessionPane: PaneAPI | undefined = $state();
	let workPane: PaneAPI | undefined = $state();

	const minSize = 5;
	const hideSize = 10;
	const sessionSize = 55;
	const workSize = 100 - sessionSize;

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

	function reset() {
		sessionPane?.resize(sessionSize);
		workPane?.resize(workSize);
	}

	import { onMount } from 'svelte';
	import type { Message, Participant, Session } from '$lib/models/session';
	let { data }: { data: { session: Session } } = $props();

	onMount(() => {
		selectedSession.set(data.session);
	});

	function messageAuthor(participantId: string): Participant {
		return (
			$selectedSession?.participants.find((p) => p.id === participantId) ?? {
				id: 'unknown',
				name: 'Anon',
				user: true
			}
		);
	}

	$effect(() => {
		if (!$selectedSession) return;
		breadcrumbs.set(['Sessions', $selectedSession.title]);
	});
</script>

{#snippet chevron(session: boolean)}
	<button
		class={`flex h-[100%] w-[100%] items-center justify-center rounded-${session ? 'l' : 'r'}-xl transition-colors hover:bg-primary/10`}
		onclick={() => reset()}
	>
		{#if session}
			<ChevronsRight />
		{:else}
			<ChevronsLeft />
		{/if}
	</button>
{/snippet}

{#snippet message(msg: Message)}
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
		<div class="flex flex-col space-y-0">
			<div class="flex items-center gap-2">
				<p class="font-bold leading-none">{participant.name}</p>
				<p class="text-sm leading-none text-muted-foreground">{formatTime(msg.timestamp)}</p>
			</div>
			<div class="pb-2 pt-2">
				<p>{msg.message}</p>
			</div>
		</div>
	</div>
{/snippet}

<div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
	{#if $selectedSession}
		<Resizable.PaneGroup direction="horizontal">
			<Resizable.Pane
				id="chat"
				class="flex flex-col"
				bind:this={sessionPane}
				defaultSize={sessionSize}
				{minSize}
			>
				{#if sessionPane?.getSize() < hideSize}
					{@render chevron(true)}
				{:else}
					<ScrollArea
						id="chat-messages"
						class="m-4 mb-2 flex h-[1px] flex-grow flex-col rounded-lg p-2"
					>
						{#each $selectedSession.chat.messages as msg}
							{@render message(msg)}
						{/each}
					</ScrollArea>

					<div id="chat-input" class="m-4 mt-2 flex flex-row rounded-lg bg-primary/10 p-2">
						<Textarea
							class="flex-1 resize-none border-none bg-transparent focus-visible:outline-none focus-visible:ring-offset-0"
							placeholder="Type your message here..."
						/>
						<div id="buttons" class="flex flex-col gap-1">
							<Button>
								<SendHorizontal />
							</Button>
							<Button>
								<Paperclip />
							</Button>
						</div>
					</div>
				{/if}
			</Resizable.Pane>
			<Resizable.Handle withHandle />
			<Resizable.Pane bind:this={workPane} defaultSize={workSize} {minSize}>
				{#if workPane?.getSize() < hideSize}
					{@render chevron(false)}
				{:else}
					arguments
				{/if}
			</Resizable.Pane>
		</Resizable.PaneGroup>
	{:else}
		<div class="flex h-[100%] w-full items-center justify-center">
			<Spinner />
		</div>
	{/if}
</div>
