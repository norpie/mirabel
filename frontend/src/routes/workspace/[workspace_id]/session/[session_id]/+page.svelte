<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import Spinner from '$lib/components/spinner.svelte';
	import ChevronsRight from 'lucide-svelte/icons/chevrons-right';
	import ChevronsLeft from 'lucide-svelte/icons/chevrons-left';

	import Chat from './chat.svelte';
	import Monitor from './monitor.svelte';

	import type { PaneAPI } from 'paneforge';
	import type { PageProps } from './$types';
	import { sessions, selectedSession, selectedWorkspace } from '$lib/store';

	import { onDestroy, onMount } from 'svelte';
	import { connectWebSocket } from '$lib/request';
	import { SessionSocketHandler } from '$lib/socket';
	import type { Chat as ChatModel, Plan } from '$lib/models/session';

	let sessionPane: PaneAPI | undefined = $state();
	let workPane: PaneAPI | undefined = $state();

	const minSize = 5;
	const hideSize = 10;
	const chatSize = 40;
	const workSize = 100 - chatSize;

	let tab = $state('spec');

	let spec: string = $state("");
	let chat: ChatModel | undefined = $state();
	let plan: Plan | undefined = $state();
	let terminal: string[] = $state([]);

	function reset() {
		sessionPane?.resize(chatSize);
		workPane?.resize(workSize);
	}

	let { data }: PageProps = $props();
	let socket: SessionSocketHandler | undefined = $state();

	onMount(async () => {
		console.log('Mounting session page with data:', data);
		sessions.set(data.sessions);
		selectedWorkspace.set(data.workspace);
		selectedSession.set(data.session);

		// Update local state variables from selectedSession when it changes
		if ($selectedSession) {
			spec = $selectedSession.plan.spec;
			plan = $selectedSession.plan;
            terminal = $selectedSession.terminal;
            chat = $selectedSession.chat;
		}

		socket = new SessionSocketHandler(connectWebSocket('v1/' + 'session/' + $selectedSession?.id));
	});

	onDestroy(async () => {
		if (!socket) return;
		console.log('Cleaning up WebSocket connection');
		socket.close();
        socket = undefined;
	});
</script>

<div class="h-full flex-1 rounded-xl bg-primary md:min-h-min">
	{#if $selectedSession}
		<Resizable.PaneGroup direction="horizontal" class="h-full">
			<Resizable.Pane
				id="chat"
				class="flex h-full flex-col"
				bind:this={sessionPane}
				defaultSize={chatSize}
				{minSize}
			>
				{#if sessionPane?.getSize() < hideSize}
					<button
						class="flex h-full w-[100%] items-center justify-center rounded-l-xl transition-colors hover:bg-secondary"
						onclick={() => reset()}
					>
						<ChevronsRight />
					</button>
				{:else}
					<Chat {socket} {chat} />
				{/if}
			</Resizable.Pane>
			<Resizable.Handle withHandle />
			<Resizable.Pane
				bind:this={workPane}
				defaultSize={workSize}
				{minSize}
				class="flex h-full flex-col"
			>
				{#if workPane?.getSize() < hideSize}
					<button
						class="flex h-full w-[100%] items-center justify-center rounded-r-xl transition-colors hover:bg-secondary"
						onclick={() => reset()}
					>
						<ChevronsLeft />
					</button>
				{:else}
					<Monitor {tab} {plan} {spec} {terminal} />
				{/if}
			</Resizable.Pane>
		</Resizable.PaneGroup>
	{:else}
		<div class="flex h-full w-full items-center justify-center">
			<Spinner />
		</div>
	{/if}
</div>
