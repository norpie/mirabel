<script lang="ts">
	import { onMount } from 'svelte';

    import { selectedSession, selectedWorkspace } from '$lib/store';

	onMount(() => {
        selectedSession.set(null);
        selectedWorkspace.set(null);
    });
</script>

<div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
    <p>Dashboard</p>
</div>
