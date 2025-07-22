<script lang="ts">
    type Props = {
        icon: HTMLElement;
        description: string;
    };

    let { icon, description } = $props();
</script>

<div class="flex items-center gap-3 py-1">
    <div class="flex h-5 w-5 flex-shrink-0 items-center justify-center">
        {icon}
    </div>
    <p class="text-sm">
        {description}
    </p>
</div>
