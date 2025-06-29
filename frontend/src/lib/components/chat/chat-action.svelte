<script lang="ts">
    type Props = {
        icon: HTMLElement,
        description: string
    };
    
    let { icon, description } = $props();
</script>

<div class="flex items-center gap-3 py-1">
    <div class="flex-shrink-0 w-5 h-5 flex items-center justify-center">
        {icon}
    </div>
    <p class="text-sm">
        {description}
    </p>
</div>