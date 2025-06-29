<script lang="ts">
    import * as Avatar from '$lib/components/ui/avatar/index.js';
	import type { Participant } from '$lib/models/session';

	let {
		acknowledgment,
		author,
		message,
	}: {
		acknowledgment?: 'delivered' | 'sent' | 'seen';
		author: Participant;
		message: string;
	} = $props();

	function formatLastChatStatus(status: 'sent' | 'delivered' | 'seen' | undefined): string {
		switch (status) {
			case 'sent':
				return 'Sent';
			case 'delivered':
				return 'Delivered';
			case 'seen':
				return 'Seen';
			default:
				return '';
		}
	}
</script>

<div class="flex items-start space-x-2 pt-2">
	<div class="flex h-[1lh] items-center justify-center">
		<Avatar.Root class="flex h-8 w-8 items-center justify-center overflow-hidden rounded-lg">
			<Avatar.Image
				src={author.avatar}
				alt={`${author.name}'s avatar`}
				class="h-full w-full object-cover"
			/>
			<Avatar.Fallback class="flex h-full w-full items-center justify-center rounded-lg">
				{author.name[0]}
			</Avatar.Fallback>
		</Avatar.Root>
	</div>
	<div>
		<p class="font-normal [overflow-wrap:anywhere]">{message}</p>
		{#if acknowledgment}
			<p class="text-muted-foreground mt-1 text-xs">
				{formatLastChatStatus(acknowledgment)}
			</p>
		{/if}
	</div>
</div>
