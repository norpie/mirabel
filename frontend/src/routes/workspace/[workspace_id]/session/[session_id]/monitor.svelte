<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import { Separator } from '$lib/components/ui/separator';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { Toggle } from '$lib/components/ui/toggle/index.js';
	import Markdown from '$lib/components/markdown.svelte';

	import Plan from '$lib/components/plan/plan.svelte';
	import Terminal from './terminal.svelte';
	import File from './file.svelte';
	import Browser from './browser.svelte';

	import SquareChartGantt from 'lucide-svelte/icons/square-chart-gantt';
	import ListTree from 'lucide-svelte/icons/list-tree';
	import SquareTerminal from 'lucide-svelte/icons/square-terminal';
	import FileStack from 'lucide-svelte/icons/file-stack';
	import Globe from 'lucide-svelte/icons/globe';
	import History from 'lucide-svelte/icons/history';

	import { selectedSession } from '$lib/store';

	let { tab }: { tab: string } = $props();

	let auto = $state(false);
</script>

<Tabs.Root bind:value={tab} class="m-2 flex h-full flex-col">
	<div class="flex flex-row justify-end">
		<div class="flex flex-row gap-3">
			<Toggle bind:pressed={auto}>Auto</Toggle>
			<Separator orientation="vertical" />
			<Tabs.List class="justify-evenly bg-transparent">
				<Tabs.Trigger value="spec" class="gap-2">
					<SquareChartGantt class="h-4 w-4" />
					<p>Spec</p></Tabs.Trigger
				>
				<Tabs.Trigger value="plan" class="gap-2">
					<ListTree class="h-4 w-4" />
					<p>Plan</p></Tabs.Trigger
				>
				<Tabs.Trigger value="terminal" class="gap-2">
					<SquareTerminal class="h-4 w-4" />
					<p>Terminal</p>
				</Tabs.Trigger>
				<Tabs.Trigger value="file" class="gap-2">
					<FileStack class="h-4 w-4" />
					<p>File</p>
				</Tabs.Trigger>
				<Tabs.Trigger value="browser" class="gap-2">
					<Globe class="h-4 w-4" />
					<p>Browser</p>
				</Tabs.Trigger>
				<Tabs.Trigger value="actions" class="gap-2">
					<History class="h-4 w-4" />
					<p>Actions</p>
				</Tabs.Trigger>
			</Tabs.List>
		</div>
	</div>
	{#if tab === 'spec'}
		<Tabs.Content
			value="spec"
			class="h-full flex-1 flex-col rounded-xl bg-secondary md:min-h-min"
		>
			<div class="flex h-full flex-col">
				<ScrollArea class="mb-2 h-[1px] flex-grow rounded-lg p-4">
					<Markdown bind:markdown={$selectedSession.plan.spec} />
				</ScrollArea>
			</div>
		</Tabs.Content>
	{/if}

	{#if tab === 'plan'}
		<Tabs.Content
			value="plan"
			class="h-full flex-1 overflow-hidden rounded-xl md:min-h-min"
		>
			<Plan bind:plan={$selectedSession.plan} />
		</Tabs.Content>
	{/if}

	{#if tab === 'terminal'}
		<Tabs.Content
			value="terminal"
			class="svelte-flow-clipping h-full flex-1 overflow-hidden rounded-xl md:min-h-min"
		>
			<Terminal />
		</Tabs.Content>
	{/if}

	{#if tab === 'file'}
		<Tabs.Content
			value="file"
			class="h-full flex-1 rounded-xl bg-primary-foreground md:min-h-min"
		>
			<File />
		</Tabs.Content>
	{/if}

	{#if tab === 'browser'}
		<Tabs.Content
			value="browser"
			class="h-full flex-1 rounded-xl bg-primary-foreground md:min-h-min"
		>
			<Browser />
		</Tabs.Content>
	{/if}

	{#if tab === 'actions'}
		<Tabs.Content
			value="actions"
			class="h-full flex-1 rounded-xl bg-primary-foreground md:min-h-min"
		></Tabs.Content>
	{/if}
</Tabs.Root>
