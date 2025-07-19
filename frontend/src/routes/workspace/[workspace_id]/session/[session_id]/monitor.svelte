<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import { Separator } from '$lib/components/ui/separator';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { Toggle } from '$lib/components/ui/toggle/index.js';
	import Markdown from '$lib/components/markdown.svelte';

	import Plan from './plan.svelte';
	import Terminal from './terminal.svelte';
	import Code from './file.svelte';
	import Browser from './browser.svelte';

	import SquareChartGantt from 'lucide-svelte/icons/square-chart-gantt';
	import ListTree from 'lucide-svelte/icons/list-tree';
	import SquareTerminal from 'lucide-svelte/icons/square-terminal';
	import Braces from 'lucide-svelte/icons/braces';
	import Globe from 'lucide-svelte/icons/globe';
	import { SessionState } from '$lib/session-state.svelte';

	let {
		tab = $bindable(),
        sessionState = $bindable(),
	}: {
        sessionState: SessionState;
		tab: string;
	} = $props();

	let spec: string | null = $state(sessionState.session?.spec ?? null);
	let plan: null = $state(null); // TODO: Fetch plan from sessionState
	let terminal: string[] | null = $state(sessionState.session?.shell ?? null);

	let auto = $state(false);

	function onTabChange() {
		if (auto) {
			auto = false;
		}
	}
</script>

<Tabs.Root bind:value={tab} onValueChange={onTabChange} class="m-2 flex h-full flex-col">
	<div class="relative w-full" style="height: 40px; overflow: hidden;">
		<div class="absolute inset-0 overflow-x-auto" style="overflow-y: hidden;">
			<Tabs.List class="flex h-full w-full flex-nowrap justify-between bg-transparent">
				<!-- Left group -->
				<div class="flex flex-nowrap gap-2">
					<Tabs.Trigger value="spec" class="gap-2">
						<SquareChartGantt class="h-4 w-4" />
						<p>Spec</p>
					</Tabs.Trigger>
					<Tabs.Trigger value="plan" class="gap-2">
						<ListTree class="h-4 w-4" />
						<p>Plan</p>
					</Tabs.Trigger>
				</div>

				<!-- Right group -->
				<div class="flex flex-nowrap gap-2">
					<Toggle bind:pressed={auto}>Auto</Toggle>
					<Separator orientation="vertical" />
					<Tabs.Trigger value="terminal" class="gap-2">
						<SquareTerminal class="h-4 w-4" />
						<p>Terminal</p>
					</Tabs.Trigger>
					<Tabs.Trigger value="file" class="gap-2">
						<Braces class="h-4 w-4" />
						<p>Code</p>
					</Tabs.Trigger>
					<Tabs.Trigger value="browser" class="gap-2">
						<Globe class="h-4 w-4" />
						<p>Browser</p>
					</Tabs.Trigger>
				</div>
			</Tabs.List>
		</div>
	</div>

	{#if tab === 'spec'}
		<Tabs.Content value="spec" class="h-full flex-1 flex-col rounded-xl md:min-h-min">
			{#if spec}
				<div class="flex h-full flex-col bg-secondary">
					<ScrollArea class="mb-2 h-[1px] flex-grow rounded-lg p-4" thumbClass="bg-zinc-600">
						<Markdown bind:markdown={spec} />
					</ScrollArea>
				</div>
			{:else}
				<div class="flex h-full w-full items-center justify-center">
					<p class="text-gray-500">No spec available.</p>
				</div>
			{/if}
		</Tabs.Content>
	{/if}

	{#if tab === 'plan'}
		<Tabs.Content
			value="plan"
			class="svelte-flow-clipping h-full flex-1 overflow-hidden rounded-xl md:min-h-min"
		>
			<Plan bind:plan />
		</Tabs.Content>
	{/if}

	{#if tab === 'terminal'}
		<Tabs.Content
			value="terminal"
			class="svelte-flow-clipping h-full flex-1 overflow-hidden rounded-xl md:min-h-min"
		>
			<Terminal lines={terminal} />
		</Tabs.Content>
	{/if}

	{#if tab === 'code'}
		<Tabs.Content
			value="code"
			class="svelte-flow-clipping h-full flex-1 overflow-hidden rounded-xl md:min-h-min"
		>
			<Code />
		</Tabs.Content>
	{/if}

	{#if tab === 'browser'}
		<Tabs.Content
			value="browser"
			class="svelte-flow-clipping h-full flex-1 overflow-hidden rounded-xl md:min-h-min"
		>
			<Browser />
		</Tabs.Content>
	{/if}
</Tabs.Root>

<style>
	:global(.svelte-flow-clipping) {
		clip-path: inset(0 round 1rem);
	}
</style>
