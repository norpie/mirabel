<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import type { PaneAPI } from 'paneforge';

	import { selectedChat, breadcrumbs } from '$lib/store';

	// Lucide
	import ChevronsLeft from 'lucide-svelte/icons/chevrons-left';
	import ChevronsRight from 'lucide-svelte/icons/chevrons-right';

	let chatPane: PaneAPI | undefined = $state();
	let workPane: PaneAPI | undefined = $state();

	const minSize = 5;
	const hideSize = 10;
	const chatSize = 55;
	const workSize = 100 - chatSize;

	function reset() {
		chatPane?.resize(chatSize);
		workPane?.resize(workSize);
	}

	$effect(() => {
		if (!$selectedChat) return;
		breadcrumbs.set(['Chats', $selectedChat.title]);
	});
</script>

{#snippet chevron(chat: boolean)}
	<button
		class={`flex h-[100%] w-[100%] items-center justify-center rounded-${chat ? 'l' : 'r'}-xl transition-colors hover:bg-primary/10`}
		onclick={() => reset()}
	>
		{#if chat}
			<ChevronsRight />
		{:else}
			<ChevronsLeft />
		{/if}
	</button>
{/snippet}

<div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
	<Resizable.PaneGroup direction="horizontal">
		<Resizable.Pane bind:this={chatPane} defaultSize={chatSize} {minSize}>
			{#if chatPane?.getSize() < hideSize}
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
</div>
