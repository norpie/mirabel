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
	import { afterNavigate, beforeNavigate } from '$app/navigation';

	let chatPane: PaneAPI | undefined = $state();
	let monitorPane: PaneAPI | undefined = $state();

	let inset: HTMLDivElement | undefined = $state();

	const minSize = 5;
	const maxSize = 100 - minSize;
	const hideSize = 15;
	const chatSize = 40;
	const monitorSize = 100 - chatSize;
	const smallScreenSize = 768;

	let isSmallScreen = $state(false);
	let disableResize = $derived(isSmallScreen);
	let enableHandle = $derived(!disableResize);

	let tab = $state('spec');

	let spec: string | undefined = $state();
	let chat: ChatModel | undefined = $state();
	let plan: Plan | undefined = $state();
	let terminal: string[] | undefined = $state([]);

	function handleResize() {
		if (!inset) return;
		if (inset?.clientWidth < smallScreenSize && !isSmallScreen) {
			isSmallScreen = true;
			chatPane?.resize(maxSize);
			monitorPane?.resize(minSize);
		} else if (inset.clientWidth >= smallScreenSize && isSmallScreen) {
			isSmallScreen = false;
			chatPane?.resize(chatSize);
			monitorPane?.resize(monitorSize);
		}
	}

	function switchSide() {
		if (isSmallScreen) {
			if (chatPane?.getSize() == maxSize) {
				chatPane?.resize(minSize);
				monitorPane?.resize(maxSize);
			} else {
				chatPane?.resize(maxSize);
				monitorPane?.resize(minSize);
			}
		} else {
			chatPane?.resize(chatSize);
			monitorPane?.resize(monitorSize);
		}
	}

	let { data }: PageProps = $props();
	let socket: SessionSocketHandler | undefined = $state();
	let socketStatus: 'open' | 'closed' | 'connecting' | 'error' = $state('closed');

	afterNavigate(async () => {
		window.addEventListener('resize', handleResize);
		sessions.set(data.sessions);
		selectedWorkspace.set(data.workspace);
		selectedSession.set(data.session);
		console.log('Session page mounted', data.session);

		if ($selectedSession) {
			plan = $selectedSession.plan;
			if (plan) {
				spec = plan.spec;
			}
			terminal = $selectedSession.terminal;
			chat = $selectedSession.chat;
			socket = connectWebSocket('v1/' + 'session/' + $selectedSession.id, undefined, (status) => {
				socketStatus = status;
			});
            console.log('WebSocket connected for session', $selectedSession.id, "and socket", socket.id);
		}
	});

	function cleanup() {
		window.removeEventListener('resize', handleResize);
		console.log('Session page destroyed, socket closed');
		if (!socket) return;
		socket.close();
	}

	beforeNavigate(cleanup);
    onDestroy(cleanup);
</script>

<div bind:this={inset} class="h-full rounded-xl bg-primary md:min-h-min">
	{#if $selectedSession}
		<Resizable.PaneGroup direction="horizontal" class="h-full">
			<Resizable.Pane
				id="chat"
				class="flex h-full flex-col"
				bind:this={chatPane}
				defaultSize={chatSize}
				{minSize}
			>
				{#if chatPane?.getSize() < hideSize}
					<button
						class="flex h-full w-full items-center justify-center rounded-l-xl transition-colors hover:bg-secondary"
						onclick={() => switchSide()}
					>
						<ChevronsRight />
					</button>
				{:else}
					<Chat {socket} {socketStatus} {chat} />
				{/if}
			</Resizable.Pane>
			<Resizable.Handle withHandle={enableHandle} disabled={disableResize} />
			<Resizable.Pane
				bind:this={monitorPane}
				defaultSize={monitorSize}
				{minSize}
				class="flex h-full flex-col"
			>
				{#if monitorPane?.getSize() < hideSize}
					<button
						class="flex h-full w-full items-center justify-center rounded-r-xl transition-colors hover:bg-secondary"
						onclick={() => switchSide()}
					>
						<ChevronsLeft />
					</button>
				{:else}
					<Monitor bind:tab {plan} {spec} {terminal} />
				{/if}
			</Resizable.Pane>
		</Resizable.PaneGroup>
	{:else}
		<div class="flex h-full w-full items-center justify-center">
			<Spinner />
		</div>
	{/if}
</div>
