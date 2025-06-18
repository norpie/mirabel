<script lang="ts">
	import * as Button from '$lib/components/ui/button/index.js';
	import FrownIcon from 'lucide-svelte/icons/frown';
	import HomeIcon from 'lucide-svelte/icons/home';
</script>

<div class="flex h-screen flex-col items-center justify-center bg-background px-4 text-foreground">
	<div class="flex max-w-md flex-col items-center space-y-8 text-center">
		<div class="flex h-24 w-24 items-center justify-center rounded-full bg-muted">
			<FrownIcon class="h-12 w-12 text-primary" strokeWidth={1.5} />
		</div>

		<div class="space-y-3">
			<h1 class="text-4xl font-bold tracking-tight">404</h1>
			<h2 class="text-2xl font-semibold">Page not found</h2>

			<p class="text-muted-foreground">
				We couldn't find the page you were looking for.
			</p>
		</div>

		<Button.Root href="/" variant="default" class="gap-2">
			<HomeIcon class="h-4 w-4" />
			Return to home
		</Button.Root>
	</div>
</div>
