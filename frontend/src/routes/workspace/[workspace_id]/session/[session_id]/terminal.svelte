<script lang="ts">
	import '../../../../../../node_modules/@xterm/xterm/css/xterm.css';

	import { Terminal } from '@xterm/xterm';
	import { FitAddon } from '@xterm/addon-fit';
	import { onMount, onDestroy } from 'svelte';
	import { mode } from 'mode-watcher';

	let {
		lines
	}: {
		lines: string[] | null;
	} = $props();

	// Sample terminal output for preview/styling
	const sampleTerminalOutput = [
		'\x1b[32m➜\x1b[0m \x1b[36m~\x1b[0m ls -la',
		'total 48',
		'drwxr-xr-x  8 user user  4096 Jul 19 14:32 \x1b[34m.\x1b[0m',
		'drwxr-xr-x  3 root root  4096 Jul 15 10:15 \x1b[34m..\x1b[0m',
		'-rw-------  1 user user  2847 Jul 19 14:30 .bash_history',
		'-rw-r--r--  1 user user   220 Jul 15 10:15 .bash_logout',
		'-rw-r--r--  1 user user  3771 Jul 15 10:15 .bashrc',
		'drwx------  2 user user  4096 Jul 15 10:20 \x1b[34m.cache\x1b[0m',
		'drwxr-xr-x  3 user user  4096 Jul 15 10:20 \x1b[34m.config\x1b[0m',
		'-rw-r--r--  1 user user   807 Jul 15 10:15 .profile',
		'drwxr-xr-x  2 user user  4096 Jul 19 14:25 \x1b[34mDocuments\x1b[0m',
		'drwxr-xr-x  2 user user  4096 Jul 18 09:30 \x1b[34mDownloads\x1b[0m',
		'-rw-r--r--  1 user user  1024 Jul 17 16:45 \x1b[32mscript.sh\x1b[0m',
		'\x1b[32m➜\x1b[0m \x1b[36m~\x1b[0m cat /proc/cpuinfo | grep "model name" | head -1',
		'model name	: Intel(R) Core(TM) i7-12700K CPU @ 3.60GHz',
		'\x1b[32m➜\x1b[0m \x1b[36m~\x1b[0m ps aux | head -5',
		'USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND',
		'root           1  0.0  0.1 168416 11684 ?        Ss   10:15   0:02 /sbin/init',
		'root           2  0.0  0.0      0     0 ?        S    10:15   0:00 [kthreadd]',
		'root           3  0.0  0.0      0     0 ?        I<   10:15   0:00 [rcu_gp]',
		'root           4  0.0  0.0      0     0 ?        I<   10:15   0:00 [rcu_par_gp]',
		'\x1b[32m➜\x1b[0m \x1b[36m~\x1b[0m \x1b[31merror:\x1b[0m command not found: invalidcommand',
		'\x1b[32m➜\x1b[0m \x1b[36m~\x1b[0m echo "\x1b[33mHello World!\x1b[0m"',
		'\x1b[33mHello World!\x1b[0m',
		'\x1b[32m➜\x1b[0m \x1b[36m~\x1b[0m'
	];

	// Theme that adapts to the application's color scheme
	const lightTheme = {
		background: 'hsl(240, 4.8%, 89%)', // --background
		foreground: 'hsl(240, 10%, 3.9%)', // --foreground
		cursor: 'hsl(240, 5.9%, 10%)', // --ring
		cursorAccent: 'hsl(240, 4.8%, 89%)', // --background
		selection: 'hsl(240, 5.9%, 88%)', // --border
		selectionForeground: 'hsl(240, 10%, 3.9%)', // --foreground
		
		// Standard colors aligned with application theme
		black: 'hsl(240, 3.8%, 46.1%)', // --muted-foreground
		red: 'hsl(0, 72.22%, 50.59%)', // --destructive
		green: 'hsl(120, 60%, 45%)', // Success green
		yellow: 'hsl(45, 90%, 50%)', // Warning yellow
		blue: 'hsl(224.3, 76.3%, 48%)', // --sidebar-primary
		magenta: 'hsl(280, 70%, 55%)', // Purple accent
		cyan: 'hsl(200, 70%, 50%)', // Cyan accent
		white: 'hsl(240, 5.9%, 10%)', // --primary-foreground
		
		// Bright variants
		brightBlack: 'hsl(240, 5%, 64.9%)', // Lighter muted
		brightRed: 'hsl(0, 72.22%, 60%)', // Brighter destructive
		brightGreen: 'hsl(120, 60%, 55%)', // Brighter green
		brightYellow: 'hsl(45, 90%, 60%)', // Brighter yellow
		brightBlue: 'hsl(224.3, 76.3%, 58%)', // Brighter blue
		brightMagenta: 'hsl(280, 70%, 65%)', // Brighter purple
		brightCyan: 'hsl(200, 70%, 60%)', // Brighter cyan
		brightWhite: 'hsl(240, 10%, 3.9%)', // --foreground
	};

	const darkTheme = {
		background: 'hsl(240, 6%, 8%)', // --background dark
		foreground: 'hsl(0, 0%, 98%)', // --foreground dark
		cursor: 'hsl(240, 4.9%, 83.9%)', // --ring dark
		cursorAccent: 'hsl(240, 6%, 8%)', // --background dark
		selection: 'hsl(240, 3.7%, 15.9%)', // --border dark
		selectionForeground: 'hsl(0, 0%, 98%)', // --foreground dark
		
		// Standard colors aligned with dark application theme
		black: 'hsl(240, 3.7%, 15.9%)', // --muted dark
		red: 'hsl(0, 62.8%, 30.6%)', // --destructive dark
		green: 'hsl(120, 50%, 60%)', // Success green dark
		yellow: 'hsl(45, 80%, 65%)', // Warning yellow dark
		blue: 'hsl(224.3, 76.3%, 48%)', // --sidebar-primary dark
		magenta: 'hsl(280, 60%, 70%)', // Purple accent dark
		cyan: 'hsl(200, 60%, 65%)', // Cyan accent dark
		white: 'hsl(240, 4.8%, 95.9%)', // --sidebar-foreground dark
		
		// Bright variants
		brightBlack: 'hsl(240, 5%, 64.9%)', // --muted-foreground dark
		brightRed: 'hsl(0, 62.8%, 40%)', // Brighter destructive dark
		brightGreen: 'hsl(120, 50%, 70%)', // Brighter green dark
		brightYellow: 'hsl(45, 80%, 75%)', // Brighter yellow dark
		brightBlue: 'hsl(224.3, 76.3%, 58%)', // Brighter blue dark
		brightMagenta: 'hsl(280, 60%, 80%)', // Brighter purple dark
		brightCyan: 'hsl(200, 60%, 75%)', // Brighter cyan dark
		brightWhite: 'hsl(0, 0%, 98%)', // --foreground dark
	};

	// Get theme based on current mode
	const getTheme = () => $mode === 'dark' ? darkTheme : lightTheme;

	let terminal: Terminal = $state(newTerminal());
	let fitAddon: FitAddon = $state(new FitAddon());
	let resizeObserver: ResizeObserver;
	let resizeTimeout: number | null = null;

	// Update terminal theme when mode changes
	// Redraw when lines change
	$effect(() => {
		redraw();
	});

	// Update terminal theme when mode changes
	$effect(() => {
		// Track mode changes
		$mode;
		
		// Update the terminal theme if it exists
		if (terminal) {
			terminal.options.theme = getTheme();
			// Force a refresh by clearing and redrawing
			redraw();
		}
	});

	$effect(() => {
		redraw();
	});

	function redraw() {
		const terminalLines = lines || sampleTerminalOutput;
        if (!terminalLines || terminalLines.length === 0) {
            terminal.clear();
            return;
        }
		terminal.clear();
		const size = terminalLines.length;
		// Write all lines with writeln except the last one
		for (let i = 0; i < size - 1; i++) {
			terminal.writeln(terminalLines[i]);
		}
		// Write the last line without writeln
		terminal.write(terminalLines[size - 1]);
		fitAddon.fit();
	}

	function newTerminal() {
		return new Terminal({
			cursorBlink: true,
			fontFamily: 'SpaceMono Nerd Font, monospace',
			fontSize: 14,
			theme: getTheme()
		});
	}

	onMount(() => {
		const terminalLines = lines || sampleTerminalOutput;
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

{#if lines || sampleTerminalOutput}
    <div id="terminal" class="flex h-full w-full overflow-hidden rounded-lg bg-card max-h-full max-w-full"></div>
{:else}
    <div class="flex items-center justify-center h-full w-full rounded bg-card">
        <p class="text-muted-foreground">No terminal output available.</p>
    </div>
{/if}

<style>
	/* Ensure the terminal container and xterm elements take full space */
	:global(.xterm) {
		height: 100%;
		width: 100%;
		max-height: 100%;
		max-width: 100%;
		padding: 0.75rem;
		box-sizing: border-box;
	}

	:global(.xterm-viewport) {
		height: 100% !important;
		width: 100% !important;
		max-height: 100% !important;
		max-width: 100% !important;
		overflow: auto !important;
	}

	:global(.xterm-screen) {
		height: 100% !important;
		width: 100% !important;
		max-height: 100% !important;
		max-width: 100% !important;
	}

	#terminal {
		display: flex;
		overflow: hidden;
		min-height: 200px;
		max-height: 100%;
		max-width: 100%;
		box-sizing: border-box;
		transition: all 150ms ease-in-out;
		contain: layout size style;
	}

	/* Ensure terminal scrollbar matches application theme */
	:global(.xterm .xterm-viewport::-webkit-scrollbar) {
		width: 8px;
	}

	:global(.xterm .xterm-viewport::-webkit-scrollbar-track) {
		background: hsl(var(--muted));
		border-radius: 4px;
	}

	:global(.xterm .xterm-viewport::-webkit-scrollbar-thumb) {
		background: hsl(var(--muted-foreground) / 0.5);
		border-radius: 4px;
	}

	:global(.xterm .xterm-viewport::-webkit-scrollbar-thumb:hover) {
		background: hsl(var(--muted-foreground) / 0.7);
	}

	/* Dark mode scrollbar adjustments */
	:global(.dark .xterm .xterm-viewport::-webkit-scrollbar-track) {
		background: hsl(var(--muted));
	}

	:global(.dark .xterm .xterm-viewport::-webkit-scrollbar-thumb) {
		background: hsl(var(--muted-foreground) / 0.6);
	}

	:global(.dark .xterm .xterm-viewport::-webkit-scrollbar-thumb:hover) {
		background: hsl(var(--muted-foreground) / 0.8);
	}
</style>
