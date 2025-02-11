<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import type { PaneAPI } from 'paneforge';

	import { selectedSession, breadcrumbs } from '$lib/store';

	import ChevronsLeft from 'lucide-svelte/icons/chevrons-left';
	import ChevronsRight from 'lucide-svelte/icons/chevrons-right';
	import Spinner from '$lib/components/spinner.svelte';

	let sessionPane: PaneAPI | undefined = $state();
	let workPane: PaneAPI | undefined = $state();

	const minSize = 5;
	const hideSize = 10;
	const sessionSize = 55;
	const workSize = 100 - sessionSize;

	function reset() {
		sessionPane?.resize(sessionSize);
		workPane?.resize(workSize);
	}

	import { onMount } from 'svelte';
	import type { Session } from '$lib/models/session';
	let { data }: { data: { session: Session } } = $props();

	onMount(() => {
		selectedSession.set(data.session);
	});

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

<div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
	{#if $selectedSession}
		<Resizable.PaneGroup direction="horizontal">
			<Resizable.Pane bind:this={sessionPane} defaultSize={sessionSize} {minSize}>
				{#if sessionPane?.getSize() < hideSize}
					{@render chevron(true)}
				{:else}
					arguments
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
