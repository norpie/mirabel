<script lang="ts">
	import '../../../../../../node_modules/@xterm/xterm/css/xterm.css';

	import { Terminal } from '@xterm/xterm';
	import { FitAddon } from '@xterm/addon-fit';
	import { onMount, onDestroy } from 'svelte';

	let {
		lines
	}: {
		lines: string[] | null;
	} = $props();

	const theme = {
		background: '#1e1e2e', // Base
		black: '#45475a', // Surface 1
		blue: '#89b4fa',
		brightBlack: '#585b70', // Surface 2
		brightBlue: '#89b4fa',
		brightCyan: '#94e2d5',
		brightGreen: '#a6e3a1',
		brightMagenta: '#f5c2e7',
		brightRed: '#f38ba8',
		brightWhite: '#a6adc8',
		brightYellow: '#f9e2af',
		cursor: '#f5e0dc', // Rosewater
		cursorAccent: '#181825', // Mantle
		cyan: '#94e2d5',
		foreground: '#cdd6f4', // Text
		green: '#a6e3a1',
		magenta: '#f5c2e7',
		red: '#f38ba8',
		selectionBackground: '#313244', // Surface 0
		selectionForeground: '#cdd6f4', // Text
		selectionInactiveBackground: '#6c7086', // Overlay 0
		white: '#bac2de', // Subtext 1
		yellow: '#f9e2af'
	};

	let terminal: Terminal = $state(newTerminal());
	let fitAddon: FitAddon = $state(new FitAddon());
	let resizeObserver: ResizeObserver;
	let resizeTimeout: number | null = null;

	$effect(() => {
		redraw();
	});

	function redraw() {
        if (!lines || lines.length === 0) {
            terminal.clear();
            return;
        }
		terminal.clear();
		const size = lines.length;
		// Write all lines with writeln except the last one
		for (let i = 0; i < size - 1; i++) {
			terminal.writeln(lines[i]);
		}
		// Write the last line without writeln
		terminal.write(lines[size - 1]);
		fitAddon.fit();
	}

	function newTerminal() {
		return new Terminal({
			cursorBlink: true,
			fontFamily: 'SpaceMono Nerd Font, monospace',
			fontSize: 14,
			theme: theme
		});
	}

	onMount(() => {
        if (!lines || lines.length === 0) {
            return;
        }
		terminal.loadAddon(fitAddon);
		let terminalElement = document.getElementById('terminal');
		if (!terminalElement) {
			console.error('Terminal element not found');
			return;
		}
		terminal.open(terminalElement);
		fitAddon.fit();

		// Add resize observer with debouncing to prevent flickering
		resizeObserver = new ResizeObserver(() => {
			// Clear any existing timeout to debounce the resize event
			if (resizeTimeout) {
				clearTimeout(resizeTimeout);
			}

			// Set a new timeout to redraw after 100ms of inactivity
			resizeTimeout = setTimeout(() => {
				redraw();
				resizeTimeout = null;
			}, 100) as unknown as number;
		});
		resizeObserver.observe(terminalElement);
	});

	onDestroy(() => {
		// Clean up observer and timeout when component is unmounted
		if (resizeObserver) {
			resizeObserver.disconnect();
		}
		if (resizeTimeout) {
			clearTimeout(resizeTimeout);
		}
	});
</script>

{#if lines}
    <div id="terminal" class="flex h-full w-full overflow-hidden"></div>
{:else}
    <div class="flex items-center justify-center h-full w-full">
        <p class="text-gray-500">No terminal output available.</p>
    </div>
{/if}

<style>
	/* Ensure the terminal container and xterm elements take full space */
	:global(.xterm) {
		height: 100%;
		width: 100%;
	}

	:global(.xterm-viewport) {
		height: 100% !important;
		width: 100% !important;
	}

	#terminal {
		display: flex;
		overflow: hidden;
	}
</style>
