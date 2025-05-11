<script lang="ts">
	import { onMount } from "svelte";
    import type { PageProps } from './$types';

    let { data }: PageProps = $props();

    import { sessions, selectedSession, selectedWorkspace } from '$lib/store';

    onMount(() => {
        sessions.set(data.sessions);
        selectedWorkspace.set(data.workspace)
        selectedSession.set(null);
    })
</script>

<div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
    <p>Workspace</p>
</div>
