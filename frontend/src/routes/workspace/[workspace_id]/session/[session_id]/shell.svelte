<script lang="ts">
	import '../../../../../../node_modules/@xterm/xterm/css/xterm.css';

	import { Terminal } from "@xterm/xterm";
    import { FitAddon } from '@xterm/addon-fit';
	import { onMount, onDestroy } from "svelte";

    let lines = $state([
        "\u001b[1;34m# Mirabel Terminal v1.0.0\u001b[0m",
        "\u001b[1;32mmirabel@workspace:~$\u001b[0m ls -la",
        "total 32",
        "drwxr-xr-x  5 mirabel users 4096 May 15 14:22 \u001b[1;34m.\u001b[0m",
        "drwxr-xr-x  3 root    root  4096 Mar 10 09:15 \u001b[1;34m..\u001b[0m",
        "-rw-------  1 mirabel users  220 Mar 10 09:15 .bash_logout",
        "-rw-------  1 mirabel users 3771 Mar 10 09:15 .bashrc",
        "drwxr-xr-x  3 mirabel users 4096 May 15 14:10 \u001b[1;34m.config\u001b[0m",
        "drwxr-xr-x  8 mirabel users 4096 May 15 14:15 \u001b[1;34mproject\u001b[0m",
        "-rw-r--r--  1 mirabel users   21 May 15 14:05 README.md",
        "\u001b[1;32mmirabel@workspace:~$\u001b[0m cd project",
        "\u001b[1;32mmirabel@workspace:~/project$\u001b[0m git status",
        "On branch \u001b[1;36mmaster\u001b[0m",
        "Your branch is up to date with '\u001b[1;36morigin/master\u001b[0m'.",
        "",
        "Changes not staged for commit:",
        "  (use \"git add <file>...\" to update what will be committed)",
        "  (use \"git restore <file>...\" to discard changes in working directory)",
        "        \u001b[1;31mmodified:   src/main.rs\u001b[0m",
        "        \u001b[1;31mmodified:   Cargo.toml\u001b[0m",
        "",
        "Untracked files:",
        "  (use \"git add <file>...\" to include in what will be committed)",
        "        \u001b[1;31msrc/utils/\u001b[0m",
        "",
        "\u001b[1;32mmirabel@workspace:~/project$\u001b[0m cargo build",
        "   \u001b[1;32mCompiling\u001b[0m project v0.1.0 (/home/mirabel/project)",
        "    \u001b[1;32mFinished\u001b[0m dev [unoptimized + debuginfo] target(s) in 1.45s",
        "\u001b[1;32mmirabel@workspace:~/project$\u001b[0m ./target/debug/project",
        "\u001b[1;33mWARNING:\u001b[0m Configuration file not found, using defaults",
        "Project initialized successfully!",
        "Listening on 127.0.0.1:8080",
        "\u001b[1;32mmirabel@workspace:~/project$\u001b[0m "
    ]);

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
    }

    let terminal: Terminal = $state(newTerminal());
    let fitAddon: FitAddon = $state(new FitAddon());
    let resizeObserver: ResizeObserver;
    let resizeTimeout: number | null = null;

    $effect(() => {
        redraw()
    });

    function redraw() {
        console.log("Terminal redraw");
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
            theme: theme,
        });
    }

    onMount(() => {
        terminal.loadAddon(fitAddon);
        let terminalElement = document.getElementById('terminal');
        if (!terminalElement) {
            console.error("Terminal element not found");
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

<div id="terminal" class="h-full w-full overflow-hidden"></div>

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
