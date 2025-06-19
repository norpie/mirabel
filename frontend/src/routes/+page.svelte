<script lang="ts">
	import { onMount } from 'svelte';
    import MoveRight from 'lucide-svelte/icons/move-right';
    import { selectedSession, selectedWorkspace, workspaces } from '$lib/store';

	onMount(() => {
        selectedSession.set(null);
        selectedWorkspace.set(null);
    });
</script>

<div class="h-full w-full flex items-center justify-center bg-primary rounded-xl">
    <div class="relative max-w-xl w-full">
        <div class="bg-secondary rounded-lg p-5 mx-auto shadow-md border border-secondary/30">
            <h3 class="mb-4 text-lg font-medium text-center">Your Workspaces</h3>
            {#if $workspaces && $workspaces.length > 0}
                <ul class="space-y-2">
                    {#each $workspaces.slice(0, 5) as workspace}
                        <li class="group">
                            <a
                                href="/workspace/{workspace.id}"
                                class="flex items-center rounded-md p-3 transition-all duration-200 w-full group-hover:translate-x-1"
                            >
                                <MoveRight class="h-4 w-4 mr-3 opacity-30 group-hover:opacity-100 transition-opacity" />
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
