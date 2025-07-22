<script lang="ts">
    import { onMount } from 'svelte';
    import MoveRight from 'lucide-svelte/icons/move-right';
    import { selectedSession, selectedWorkspace, workspaces } from '$lib/store';

    onMount(() => {
        selectedSession.set(null);
        selectedWorkspace.set(null);
    });
</script>

<div class="flex h-full w-full items-center justify-center rounded-xl bg-primary">
    <div class="relative w-full max-w-xl">
        <div class="mx-auto rounded-lg border border-secondary/30 bg-secondary p-5 shadow-md">
            <h3 class="mb-4 text-center text-lg font-medium">Your Workspaces</h3>
            {#if $workspaces && $workspaces.length > 0}
                <ul class="space-y-2">
                    {#each $workspaces.slice(0, 5) as workspace}
                        <li class="group">
                            <a
                                href="/workspace/{workspace.id}"
                                class="flex w-full items-center rounded-md p-3 transition-all duration-200 group-hover:translate-x-1"
                            >
                                <MoveRight
                                    class="mr-3 h-4 w-4 opacity-30 transition-opacity group-hover:opacity-100"
                                />
                                <span class="truncate">{workspace.name}</span>
                            </a>
                        </li>
                    {/each}
                </ul>
            {:else}
                <p class="text-center text-muted-foreground">No workspaces found</p>
            {/if}
        </div>
    </div>
</div>
