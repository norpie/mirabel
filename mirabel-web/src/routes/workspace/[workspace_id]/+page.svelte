<script lang="ts">
    import type { PageProps } from './$types';
    import { Textarea } from '$lib/components/ui/textarea/index.js';
    import { Button } from '$lib/components/ui/button/index.js';
    import { ChatInput } from '$lib/components/chat/index';
    import SendHorizontal from 'lucide-svelte/icons/send-horizontal';
    import Paperclip from 'lucide-svelte/icons/paperclip';
    import MoveRight from 'lucide-svelte/icons/move-right';
    import { toast } from 'svelte-sonner';
    import { sessions, selectedSession, selectedWorkspace } from '$lib/store';
    import { page } from '$app/state';
    import { goto } from '$app/navigation';
    import { post } from '$lib/request';
    import type { Session } from '$lib/models/session';

    let { data }: PageProps = $props();

    let chatInput = $state('');
    let workspaceId = $derived(page.params.workspace_id);

    $effect(() => {
        selectedSession.set(null);
        selectedWorkspace.set(data.workspace);
        sessions.set(data.sessions);
    });

    $inspect(data);

    async function sendMessage() {
        if (!$selectedWorkspace) {
            toast.error('Please select a workspace first.');
            return;
        }
        if (!chatInput.trim()) return;
        const session = await post<Session>(`v1/workspace/${workspaceId}/session`, {
            input: chatInput
        });
        if (!session || !session.data) {
            toast.error('Failed to create session. Please try again.');
            return;
        }
        if (session.error) {
            toast.error(`Error: ${session.error}`);
            return;
        }
        goto(`/workspace/${$selectedWorkspace.id}/session/${session.data.id}`);
    }
</script>

<div class="flex h-full w-full items-center justify-center rounded-xl bg-primary">
    <div class="relative w-full max-w-xl">
        <!-- Recent sessions above chat input -->
        {#if $sessions && $sessions.length > 0}
            <div class="absolute bottom-full left-0 right-0 mb-8">
                <div
                    class="relative mx-auto rounded-lg border border-secondary/30 bg-secondary p-5 shadow-md"
                >
                    <h3 class="mb-4 text-center text-lg font-medium">Recent Sessions</h3>
                    <ul class="space-y-2">
                        {#each $sessions.slice(0, 5) as session}
                            <li class="group">
                                <a
                                    href="/workspace/{workspaceId}/session/{session.id}"
                                    class="flex w-full items-center rounded-md p-3 transition-all duration-200 group-hover:translate-x-1"
                                >
                                    <MoveRight
                                        class="mr-3 h-4 w-4 opacity-30 transition-opacity group-hover:opacity-100"
                                    />
                                    <span class="truncate">{session.title}</span>
                                </a>
                            </li>
                        {/each}
                    </ul>
                </div>
            </div>
        {/if}

        <ChatInput bind:value={chatInput} send={sendMessage} />
    </div>
</div>
