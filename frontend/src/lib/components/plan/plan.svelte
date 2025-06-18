<script lang="ts">
	import { onMount } from 'svelte';
	import type { Plan } from '$lib/models/session';
	import { writable, type Writable } from 'svelte/store';

	import { mode } from 'mode-watcher';

	import { Background, BackgroundVariant, SvelteFlow } from '@xyflow/svelte';
	import '@xyflow/svelte/dist/style.css';
	import Elk from 'elkjs/lib/elk.bundled.js';
	import { getEdges, getNodes } from './planToFlow';

	let { plan = $bindable() }: { plan: Plan } = $props();

	let nodes: Writable<any[]> = writable([]);
	let edges: Writable<any[]> = writable([]);
	let elk = new Elk();

	async function layout() {
		const graph = {
			id: 'root',
			layoutOptions: {
				'elk.algorithm': 'layered',
				'elk.direction': 'RIGHT',
				'elk.spacing.nodeNode': 20,
				'elk.spacing.edgeNode': 50,
				'elk.layered.spacing.nodeNodeBetweenLayers': 50
			},
			children: getNodes(plan),
			edges: getEdges(plan)
		};

		const laidOutGraph = await elk.layout(graph);

		if (!laidOutGraph || !laidOutGraph.children || !laidOutGraph.edges) return;

		// Find max y-value to flip the vertical orientation
		const maxY = Math.max(...laidOutGraph.children.map((node) => node.y));

		// Update node positions
		nodes.set(
			laidOutGraph.children.map((node) => ({
				id: node.id,
				type: node.type,
				targetPosition: 'left',
				sourcePosition: 'right',
				position: { x: node.x, y: maxY - node.y },
				data: { label: node.label }
			}))
		);

		edges.set(
			laidOutGraph.edges.map((edge) => ({
				id: edge.id,
				source: edge.source,
				target: edge.target,
				animated: edge.status === 'in-progress',
				label: label(edge.status)
			}))
		);
	}

	function type(
		source: string | undefined,
		target: string | undefined
	): 'default' | 'input' | 'output' {
		if (source === undefined) return 'input';
		if (target === undefined) return 'output';
		return 'default';
	}

	function label(status: string): string {
		switch (status) {
			case 'in-progress':
				return '';
			case 'done':
				return '';
			case 'failed':
				return '';
			case 'paused':
				return '';
			case 'todo':
				return '';
			default:
				return '?';
		}
	}

	const proOptions = { hideAttribution: true };

	onMount(layout);
</script>

<SvelteFlow class="svelte-flow-clipping font-mono" {nodes} {edges} colorMode={$mode} fitView {proOptions}>
	<Background variant={BackgroundVariant.Dots} />
</SvelteFlow>
