<script lang="ts">
    import { type Plan, placeholderPlan } from '$lib/models/plan';
    import PlanTask from './plan-task.svelte';
    import { ScrollArea } from '$lib/components/ui/scroll-area';
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { Badge } from '$lib/components/ui/badge';
    import Activity from 'lucide-svelte/icons/activity';

    let { plan }: { plan: Plan | null } = $props();

    // TODO: Remove this when we implement the API to fetch the plan.
    $effect(() => {
        plan = placeholderPlan();
    });

    let stats = $derived.by(() => {
        if (!plan) return { total: 0, completed: 0, paused: 0 };
        
        function countTasks(tasks: PlanTask[]): { total: number, completed: number, paused: number } {
            let total = 0;
            let completed = 0;
            let paused = 0;
            
            for (const task of tasks) {
                total++;
                if (task.status === 'completed') completed++;
                if (task.status === 'paused') paused++;
                
                const childStats = countTasks(task.children);
                total += childStats.total;
                completed += childStats.completed;
                paused += childStats.paused;
            }
            
            return { total, completed, paused };
        }
        
        return countTasks(plan.tasks);
    });

    // Control functions for expanding/collapsing tasks
    function handleExpandTask(taskId: string) {
        console.log(`Expanding task: ${taskId}`);
        // Can be used for analytics, state management, etc.
    }

    function handleCollapseTask(taskId: string) {
        console.log(`Collapsing task: ${taskId}`);
        // Can be used for analytics, state management, etc.
    }

    // Utility functions that could be called from parent components
    export function expandAllTasks() {
        // Implementation would require task refs - for now just log
        console.log('Expanding all tasks');
    }

    export function collapseAllTasks() {
        // Implementation would require task refs - for now just log
        console.log('Collapsing all tasks');
    }

    export function expandToCurrentTask() {
        // Implementation would expand path to current task
        console.log('Expanding to current task');
    }
</script>

{#if plan}
    <div class="flex h-full w-full flex-col gap-2">
        <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
                <h2 class="text-xl font-semibold">
                    {plan.title}
                </h2>
                <Badge variant="secondary">{stats.completed}/{stats.total} completed</Badge>
            </div>
        </div>
        
        <ScrollArea class="h-[1px] flex-grow" fadeout="both">
            <div class="space-y-2 p-1">
                {#each plan.tasks as task (task.id)}
                    <PlanTask {task} {plan} onExpandTask={handleExpandTask} onCollapseTask={handleCollapseTask} />
                {/each}
            </div>
        </ScrollArea>
    </div>
{:else}
    <div class="flex h-full w-full items-center justify-center">
        <p class="text-gray-500 dark:text-gray-400">No plan available.</p>
    </div>
{/if}
