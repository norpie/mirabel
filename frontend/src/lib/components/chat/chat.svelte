<script lang="ts">
	let { children } = $props();
</script>

<div id="chat" class="flex h-full flex-col">
	{@render children()}
</div>