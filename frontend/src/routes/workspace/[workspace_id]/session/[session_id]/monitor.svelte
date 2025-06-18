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

	let { tab = $bindable(), plan, spec, terminal }: { tab: string; plan: any; spec: string; terminal: string[] } =
		$props();

	let auto = $state(false);

    function onTabChange() {
        if(auto) {
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
					<Tabs.Trigger value="actions" class="gap-2">
						<History class="h-4 w-4" />
						<p>Actions</p>
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
						<FileStack class="h-4 w-4" />
						<p>File</p>
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
		<Tabs.Content value="spec" class="h-full flex-1 flex-col rounded-xl bg-secondary md:min-h-min">
			<div class="flex h-full flex-col">
				<ScrollArea class="mb-2 h-[1px] flex-grow rounded-lg p-4" thumbClass="bg-zinc-600" >
					<Markdown bind:markdown={spec} />
				</ScrollArea>
			</div>
		</Tabs.Content>
	{/if}

	{#if tab === 'plan'}
		<Tabs.Content value="plan" class="svelte-flow-clipping h-full flex-1 overflow-hidden rounded-xl md:min-h-min">
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

	{#if tab === 'file'}
		<Tabs.Content
			value="file"
			class="svelte-flow-clipping h-full flex-1 overflow-hidden rounded-xl md:min-h-min"
		>
			<File />
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

	{#if tab === 'actions'}
		<Tabs.Content
			value="actions"
			class="svelte-flow-clipping h-full flex-1 overflow-hidden rounded-xl md:min-h-min"
		></Tabs.Content>
	{/if}
</Tabs.Root>


<style>
:global(.svelte-flow-clipping) {
    clip-path: inset(0 round 1rem);
}
</style>
