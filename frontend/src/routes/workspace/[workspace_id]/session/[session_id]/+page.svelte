<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import Spinner from '$lib/components/spinner.svelte';

	import Chat from './chat.svelte';
	import Monitor from './monitor.svelte';

	import type { PaneAPI } from 'paneforge';
	import type { PageProps } from './$types';
	import { sessions, selectedSession, selectedWorkspace } from '$lib/store';

	import { onDestroy, onMount } from 'svelte';
	import { connectWebSocket } from '$lib/request';
	import { SessionSocketHandler } from '$lib/socket';

	let sessionPane: PaneAPI | undefined = $state();
	let workPane: PaneAPI | undefined = $state();

	const minSize = 5;
	const chatSize = 40;
	const workSize = 100 - chatSize;

    let tab = $state('spec');

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
		socket = new SessionSocketHandler(connectWebSocket('v1/' + 'session/' + $selectedSession?.id));
	});

	onDestroy(async () => {
        if (!socket) return;
		console.log('Cleaning up WebSocket connection');
		socket.close();
	});

	$effect(() => {
		if (!$selectedSession) return;
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
				<Chat {sessionPane} {reset} {socket} />
			</Resizable.Pane>
			<Resizable.Handle withHandle />
			<Resizable.Pane
				bind:this={workPane}
				defaultSize={workSize}
				{minSize}
				class="flex h-full flex-col"
			>
				<Monitor {workPane} {reset} bind:tab/>
			</Resizable.Pane>
		</Resizable.PaneGroup>
	{:else}
		<div class="flex h-full w-full items-center justify-center">
			<Spinner />
		</div>
	{/if}
</div>
