<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import SendHorizontal from 'lucide-svelte/icons/send-horizontal';
	import Paperclip from 'lucide-svelte/icons/paperclip';
    import MoveRight from 'lucide-svelte/icons/move-right';
	import { toast } from 'svelte-sonner';
	import { sessions, selectedSession, selectedWorkspace } from '$lib/store';
	import { page } from '$app/state';

	let { data }: PageProps = $props();
	let chatInput = $state('');
	let workspaceId = $derived(page.params.workspace_id);

	onMount(() => {
		selectedWorkspace.set(data.workspace);
		sessions.set(data.sessions);
		selectedSession.set(null);
	});

	async function sendMessage() {
		if (!chatInput.trim()) return;

		// Placeholder for actual message sending functionality
		toast.success(`Message sent: ${chatInput}`);

		console.log('Sending message:', chatInput);
		chatInput = '';
	}
</script>

<div class="h-full w-full flex items-center justify-center bg-primary rounded-xl">
    <div class="relative max-w-xl w-full">
        <!-- Recent sessions above chat input -->
        {#if $sessions && $sessions.length > 0}
            <div class="absolute bottom-full left-0 right-0 mb-8">
                <div class="bg-secondary relative rounded-lg p-5 mx-auto shadow-md border border-secondary/30">
                    <h3 class="mb-4 text-lg font-medium text-center">Recent Sessions</h3>
                    <ul class="space-y-2">
                        {#each $sessions.slice(0, 5) as session}
                            <li class="group">
                                <a
                                    href="/workspace/{workspaceId}/session/{session.id}"
                                    class="flex items-center rounded-md p-3 transition-all duration-200 w-full group-hover:translate-x-1"
                                >
                                    <MoveRight class="h-4 w-4 mr-3 opacity-30 group-hover:opacity-100 transition-opacity" />
                                    <span class="truncate">{session.title}</span>
                                </a>
                            </li>
                        {/each}
                    </ul>
                </div>
            </div>
        {/if}

        <!-- Chat input section -->
        <div class="flex items-center flex-row bg-secondary rounded-lg p-2 mx-auto">
            <Textarea
                class="flex-1 resize-none border-none bg-transparent focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-offset-0"
                placeholder="Type your message here..."
                bind:value={chatInput}
                onkeydown={(e) => {
                    if (e.key === 'Enter' && !e.shiftKey) {
                        sendMessage();
                        e.preventDefault();
                    }
                }}
            />
            <div class="flex flex-col gap-1 pl-2">
                <Button onclick={() => sendMessage()}>
                    <SendHorizontal class="pointer-events-none" />
                </Button>
                <Button>
                    <Paperclip />
                </Button>
            </div>
        </div>
    </div>
</div>
