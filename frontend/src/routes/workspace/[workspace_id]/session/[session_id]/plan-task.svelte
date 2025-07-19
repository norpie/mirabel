<script lang="ts">
    import type { PlanTask, Plan } from '$lib/models/plan';
    import { Badge } from '$lib/components/ui/badge';
    import { Card } from '$lib/components/ui/card';
    import Spinner from '$lib/components/throbbers/spinner.svelte';
    import Check from 'lucide-svelte/icons/check';
    import Clock from 'lucide-svelte/icons/clock';
    import Play from 'lucide-svelte/icons/play';
    import Pause from 'lucide-svelte/icons/pause';
    import X from 'lucide-svelte/icons/x';
    import CornerDownRight from 'lucide-svelte/icons/corner-down-right';
    import ArrowRight from 'lucide-svelte/icons/arrow-right';
    import ChevronRight from 'lucide-svelte/icons/chevron-right';
    import ChevronDown from 'lucide-svelte/icons/chevron-down';

    let {
        task,
        plan,
        depth = 0,
        onExpandTask,
        onCollapseTask
    }: {
        task: PlanTask;
        plan: Plan;
        depth?: number;
        onExpandTask?: (taskId: string) => void;
        onCollapseTask?: (taskId: string) => void;
    } = $props();

    // Helper function to check if task or any child is running
    function hasRunningTask(task: PlanTask): boolean {
        if (task.status === 'in-progress') return true;
        return task.children.some((child) => hasRunningTask(child));
    }

    // Initial expansion state: only expand if this task or its children have running tasks
    let expanded = $state(hasRunningTask(task));

    // Functions for programmatic control
    function expandTask() {
        expanded = true;
        onExpandTask?.(task.id);
    }

    function collapseTask() {
        expanded = false;
        onCollapseTask?.(task.id);
    }

    function toggleExpansion() {
        if (expanded) {
            collapseTask();
        } else {
            expandTask();
        }
    }

    function getStatusIcon(status: string) {
        switch (status) {
            case 'completed':
                return Check;
            case 'in-progress':
                return Play;
            case 'paused':
                return Pause;
            case 'cancelled':
                return X;
            default:
                return Clock;
        }
    }

    function getStatusColor(status: string) {
        switch (status) {
            case 'completed':
                return 'bg-green-100 border-green-200 text-green-700 dark:bg-green-900/20 dark:border-green-800 dark:text-green-300';
            case 'in-progress':
                return 'bg-blue-100 border-blue-200 text-blue-700 dark:bg-blue-900/20 dark:border-blue-800 dark:text-blue-300';
            case 'paused':
                return 'bg-yellow-100 border-yellow-200 text-yellow-700 dark:bg-yellow-900/20 dark:border-yellow-800 dark:text-yellow-300';
            case 'cancelled':
                return 'bg-red-100 border-red-200 text-red-500 dark:bg-red-900/20 dark:border-red-800 dark:text-red-400';
            default:
                return 'bg-gray-50 border-gray-200 text-gray-600 dark:bg-gray-800/50 dark:border-gray-700 dark:text-gray-300';
        }
    }

    function getTypeIcon(type: string) {
        switch (type) {
            case 'detour':
                return CornerDownRight;
            case 'deviation':
                return ArrowRight;
            default:
                return null;
        }
    }

    function getTypeColor(type: string) {
        switch (type) {
            case 'detour':
                return 'bg-yellow-100 text-yellow-700 border-yellow-200 dark:bg-yellow-900/20 dark:text-yellow-300 dark:border-yellow-800';
            case 'deviation':
                return 'bg-orange-100 text-orange-700 border-orange-200 dark:bg-orange-900/20 dark:text-orange-300 dark:border-orange-800';
            default:
                return 'bg-blue-100 text-blue-700 border-blue-200 dark:bg-blue-900/20 dark:text-blue-300 dark:border-blue-800';
        }
    }

    let StatusIcon = $derived(getStatusIcon(task.status));
    let TypeIcon = $derived(getTypeIcon(task.type));
    let isCurrentTask = $derived(plan.currentTaskId === task.id);
</script>

<!-- UI Control Snippets -->
{#snippet expansionButton()}
    {#if task.children.length > 0}
        <button
            class="flex h-5 w-5 items-center justify-center rounded hover:bg-white/50 dark:hover:bg-gray-700/50"
            onclick={toggleExpansion}
        >
            {#if expanded}
                <ChevronDown class="h-4 w-4" />
            {:else}
                <ChevronRight class="h-4 w-4" />
            {/if}
        </button>
    {:else}
        <div class="h-5 w-5"></div>
    {/if}
{/snippet}

{#snippet taskCheckbox()}
    <div
        class="flex h-5 w-5 flex-shrink-0 items-center justify-center rounded border-2 {task.status ===
        'cancelled'
            ? 'border-red-400 bg-red-100 dark:border-red-600 dark:bg-red-900/30'
            : isCurrentTask
              ? 'border-blue-400 bg-blue-100 dark:border-blue-500 dark:bg-blue-900/30'
              : task.status === 'completed'
                ? 'border-green-400 bg-green-100 dark:border-green-500 dark:bg-green-900/30'
                : task.status === 'in-progress'
                  ? 'border-blue-400 bg-blue-100 dark:border-blue-500 dark:bg-blue-900/30'
                  : task.status === 'paused'
                    ? 'border-yellow-400 bg-yellow-100 dark:border-yellow-500 dark:bg-yellow-900/30'
                    : 'border-gray-300 bg-white dark:border-gray-600 dark:bg-gray-800'}"
    >
        {#if task.status === 'completed'}
            <svg
                class="h-3 w-3 text-green-600 dark:text-green-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="3"
            >
                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
            </svg>
        {:else if task.status === 'in-progress'}
            <div class="flex h-3 w-3 items-center justify-center">
                <svg
                    class="spinner h-3 w-3 text-blue-600 dark:text-blue-400"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <circle
                        class="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        stroke-width="4"
                    ></circle>
                    <path
                        class="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    ></path>
                </svg>
            </div>
        {:else if task.status === 'paused'}
            <Pause
                class="h-2.5 w-2.5 {isCurrentTask
                    ? 'text-blue-600 dark:text-blue-400'
                    : 'text-yellow-600 dark:text-yellow-400'}"
            />
        {/if}
    </div>
{/snippet}

<!-- Content Snippets -->
{#snippet taskTitle()}
    <div class="flex flex-wrap items-center gap-2">
        <span class="text-sm font-medium">{task.title}</span>

        {#if task.type !== 'planned'}
            <Badge variant="outline" class="text-xs {getTypeColor(task.type)}">
                {#if TypeIcon}
                    <TypeIcon class="mr-1 h-3 w-3" />
                {/if}
                {task.type === 'detour' ? 'Detour' : 'Deviation'}
            </Badge>
        {/if}
    </div>
{/snippet}

{#snippet taskDetourReason()}
    {#if task.detourReason}
        <p
            class="inline-block rounded border border-amber-200 bg-amber-50 px-2 py-1 text-xs italic text-amber-700 dark:border-amber-800 dark:bg-amber-900/20 dark:text-amber-300"
        >
            <strong>Detour Reason:</strong>
            {task.detourReason}
        </p>
    {/if}
    {#if task.deviationReason}
        <p
            class="inline-block rounded border border-orange-200 bg-orange-50 px-2 py-1 text-xs italic text-orange-700 dark:border-orange-800 dark:bg-orange-900/20 dark:text-orange-300"
        >
            <strong>Deviation Reason:</strong>
            {task.deviationReason}
        </p>
    {/if}
{/snippet}

<!-- Layout Snippets -->
{#snippet taskMainContent()}
    <div class="flex items-start gap-3 {task.status === 'cancelled' ? 'relative' : ''}">
        <div class="relative">
            <div class="flex items-start gap-2 p-1">
                {@render taskCheckbox()}
                {@render taskTitle()}
            </div>
            {#if task.status === 'cancelled'}
                <span
                    class="pointer-events-none absolute left-0 right-0 top-1/2 h-0.5 -translate-y-1/2 bg-red-400 dark:bg-red-500"
                ></span>
            {/if}
        </div>
    </div>
{/snippet}

{#snippet taskSecondaryContent()}
    {#if task.detourReason || task.deviationReason}
        <div class="flex items-start gap-3">
            <div class="flex-1">
                {@render taskDetourReason()}
            </div>
        </div>
    {/if}
{/snippet}

<div class="space-y-2" style="margin-left: {depth}rem">
    <div
        class="flex items-start gap-3 rounded-lg border p-2 transition-all {isCurrentTask
            ? 'border-blue-200 bg-blue-100 text-blue-700 dark:border-blue-600 dark:bg-blue-900/30 dark:text-blue-300'
            : getStatusColor(task.status)} {isCurrentTask
            ? 'shadow-md ring-2 ring-blue-400 dark:ring-blue-500'
            : ''}"
    >
        <div class="mt-0.5 flex-shrink-0">
            {@render expansionButton()}
        </div>

        <div class="min-w-0 flex-1 space-y-1">
            {@render taskMainContent()}
            {@render taskSecondaryContent()}
        </div>
    </div>

    {#if expanded && task.children.length > 0}
        <div class="ml-3 space-y-2">
            {#each task.children as child (child.id)}
                <svelte:self
                    task={child}
                    {plan}
                    depth={depth + 1}
                    {onExpandTask}
                    {onCollapseTask}
                />
            {/each}
        </div>
    {/if}
</div>

<style>
    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    .spinner {
        animation: spin 1.5s linear infinite;
    }
</style>
