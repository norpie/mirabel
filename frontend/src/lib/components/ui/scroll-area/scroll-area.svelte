<script lang="ts">
	import { ScrollArea as ScrollAreaPrimitive, type WithoutChild } from "bits-ui";
	import { Scrollbar } from "./index.js";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
        viewportRef = $bindable(null),
		class: className,
		orientation = "vertical",
		scrollbarXClasses = "",
		scrollbarYClasses = "",
        thumbClass = "",
		fadeout = "none",
		children,
		...restProps
	}: WithoutChild<ScrollAreaPrimitive.RootProps> & {
        viewportRef?: HTMLElement | null | undefined;
		orientation?: "vertical" | "horizontal" | "both" | undefined;
		scrollbarXClasses?: string | undefined;
		scrollbarYClasses?: string | undefined;
        thumbClass?: string | undefined;
		fadeout?: "none" | "both" | "top" | "bottom" | undefined;
	} = $props();

	let showTopFade = $state(false);
	let showBottomFade = $state(false);

	function updateFadeVisibility() {
		if (!viewportRef || fadeout === "none") return;
		
		const { scrollTop, scrollHeight, clientHeight } = viewportRef;
		const isAtTop = scrollTop <= 1; // Small tolerance for floating point precision
		const isAtBottom = scrollTop + clientHeight >= scrollHeight - 1;
		
		if (fadeout === "top" || fadeout === "both") {
			showTopFade = !isAtTop;
		}
		
		if (fadeout === "bottom" || fadeout === "both") {
			showBottomFade = !isAtBottom;
		}
	}

	$effect(() => {
		if (viewportRef && fadeout !== "none") {
			viewportRef.addEventListener('scroll', updateFadeVisibility);
			// Initial check
			updateFadeVisibility();
			
			return () => {
				viewportRef?.removeEventListener('scroll', updateFadeVisibility);
			};
		}
	});

	function getFadeoutClasses() {
		let classes = "";
		
		if (showTopFade) {
			classes += " before:absolute before:top-0 before:left-0 before:right-0 before:h-4 before:bg-gradient-to-b before:from-background before:to-transparent before:z-10 before:pointer-events-none";
		}
		
		if (showBottomFade) {
			classes += " after:absolute after:bottom-0 after:left-0 after:right-0 after:h-4 after:bg-gradient-to-t after:from-background after:to-transparent after:z-10 after:pointer-events-none";
		}
		
		return classes;
	}
</script>

<ScrollAreaPrimitive.Root bind:ref {...restProps} class={cn("relative overflow-hidden", getFadeoutClasses(), className)}>
	<ScrollAreaPrimitive.Viewport bind:ref={viewportRef} class="h-full w-full rounded-[inherit]">
		{@render children?.()}
	</ScrollAreaPrimitive.Viewport>
	{#if orientation === "vertical" || orientation === "both"}
		<Scrollbar orientation="vertical" {thumbClass} class={scrollbarYClasses} />
	{/if}
	{#if orientation === "horizontal" || orientation === "both"}
		<Scrollbar orientation="horizontal" {thumbClass} class={scrollbarXClasses} />
	{/if}
	<ScrollAreaPrimitive.Corner />
</ScrollAreaPrimitive.Root>
