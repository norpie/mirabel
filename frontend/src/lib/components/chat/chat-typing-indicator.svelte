<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import Mirabel from '$lib/assets/mirabel.png';
	import Spinner from '$lib/components/throbbers/spinner.svelte';
	import Bouncer from '$lib/components/throbbers/bouncer.svelte';
	import Pause from 'lucide-svelte/icons/pause';
	import { formatElapsedTime } from '$lib/time';

	let {
		mirabelStatus,
		lastAcknowledgementTime
	}: {
		mirabelStatus: 'thinking' | 'typing' | 'paused' | 'error';
		lastAcknowledgementTime: Date;
	} = $props();

	let currentTime: Date = $state(new Date());
	let elapsedTime: string = $derived.by(() => {
		return formatElapsedTime(lastAcknowledgementTime ?? new Date(), currentTime);
	});

	let timer: number;

	$effect(() => {
		timer = window.setInterval(() => {
			currentTime = new Date();
		}, 1000);

		return () => {
			clearInterval(timer);
		};
	});

	$inspect(elapsedTime);
</script>

<div class="mb-4 flex space-x-4">
	<Avatar.Root class="h-8 w-8 rounded-lg">
		<Avatar.Image src={Mirabel} alt={`Mirabel's avatar`} />
		<Avatar.Fallback class="rounded-lg">M</Avatar.Fallback>
	</Avatar.Root>
	<div class="flex flex-col justify-center">
		<div>
			<div class="flex items-center">
				{#if mirabelStatus === 'thinking'}
					<Spinner />
				{:else if mirabelStatus === 'typing'}
					<Bouncer />
				{:else if mirabelStatus === 'paused'}
					<!-- Pause icon for paused status -->
					<div class="flex h-5 w-5 items-center justify-center">
						<Pause class="h-4 w-4 text-muted-foreground" />
					</div>
				{/if}
				<span class="ml-3 text-sm text-muted-foreground">
					{#if mirabelStatus === 'thinking'}
						Thinking for {elapsedTime}...
					{:else if mirabelStatus === 'typing'}
						typing for {elapsedTime}...
					{:else if mirabelStatus === 'paused'}
						Paused for {elapsedTime}
					{/if}
				</span>
			</div>
		</div>
	</div>
</div>
