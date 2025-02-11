<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import Ellipsis from 'lucide-svelte/icons/ellipsis';
	import Eye from 'lucide-svelte/icons/eye';
	import Star from 'lucide-svelte/icons/star';
	import Archive from 'lucide-svelte/icons/archive';

	import { chats } from '$lib/store';
	import Spinner from './spinner.svelte';

	const sidebar = useSidebar();
</script>

<Sidebar.Group class="group-data-[collapsible=icon]:hidden">
	<Sidebar.GroupLabel>Chats</Sidebar.GroupLabel>
	<Sidebar.Menu>
		{#if $chats}
			{#each $chats as chat (chat.title)}
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<button {...props}>
								<span>{chat.title}</span>
							</button>
						{/snippet}
					</Sidebar.MenuButton>
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							{#snippet child({ props })}
								<Sidebar.MenuAction showOnHover {...props}>
									<Ellipsis />
								</Sidebar.MenuAction>
							{/snippet}
						</DropdownMenu.Trigger>
						<DropdownMenu.Content
							class="w-48 rounded-lg"
							side={sidebar.isMobile ? 'bottom' : 'right'}
							align={sidebar.isMobile ? 'end' : 'start'}
						>
							<DropdownMenu.Item>
								<Eye class="text-muted-foreground" />
								<span>View Chat</span>
							</DropdownMenu.Item>
							<DropdownMenu.Item>
								<Star class="text-muted-foreground" />
								<span>Favourite Chat</span>
							</DropdownMenu.Item>
							<DropdownMenu.Separator />
							<DropdownMenu.Item>
								<Archive class="text-muted-foreground" />
								<span>Archive Chat</span>
							</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</Sidebar.MenuItem>
			{/each}
		{:else}
			<Sidebar.MenuItem>
				<Spinner />
			</Sidebar.MenuItem>
		{/if}
	</Sidebar.Menu>
</Sidebar.Group>
