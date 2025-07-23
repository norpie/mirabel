<script lang="ts">
    import * as Card from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Label } from '$lib/components/ui/label';
    import { Separator } from '$lib/components/ui/separator';
    import { Badge } from '$lib/components/ui/badge';
    import { Monitor, Moon, Sun, Palette } from 'lucide-svelte';
    import { mode, setMode } from 'mode-watcher';

    type Theme = 'light' | 'dark' | 'system';

    let selectedTheme = $state<Theme>('system');
    let isLoading = $state(false);

    // Sync with mode-watcher
    $effect(() => {
        if ($mode === 'light') selectedTheme = 'light';
        else if ($mode === 'dark') selectedTheme = 'dark';
        else selectedTheme = 'system';
    });

    async function handleThemeChange(theme: Theme) {
        selectedTheme = theme;
        setMode(theme);

        // TODO: Save preference to backend
        try {
            console.log('Saving theme preference:', theme);
        } catch (error) {
            console.error('Failed to save theme preference:', error);
        }
    }

    const themes = [
        {
            value: 'light' as const,
            label: 'Light',
            description: 'Clean and bright interface',
            icon: Sun
        },
        {
            value: 'dark' as const,
            label: 'Dark',
            description: 'Easy on the eyes',
            icon: Moon
        },
        {
            value: 'system' as const,
            label: 'System',
            description: 'Follow system preference',
            icon: Monitor
        }
    ];
</script>

<div class="flex h-full flex-col">
    <div class="flex-none space-y-1 px-6 pb-4 pt-2">
        <h2 class="text-2xl font-bold tracking-tight">Appearance Settings</h2>
        <p class="text-muted-foreground">Customize the look and feel of your interface.</p>
    </div>

    <Separator class="flex-none" />

    <div class="flex-1 space-y-6 p-6">
        <!-- Theme Selection -->
        <Card.Root>
            <Card.Header>
                <Card.Title>Theme</Card.Title>
                <Card.Description>Choose your preferred color scheme</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
                    {#each themes as theme (theme.value)}
                        <button
                            class="relative flex flex-col items-center space-y-3 rounded-lg border p-4 transition-colors hover:bg-muted {selectedTheme ===
                            theme.value
                                ? 'border-primary bg-muted'
                                : 'border-border'}"
                            onclick={() => handleThemeChange(theme.value)}
                        >
                            <div
                                class="flex h-12 w-12 items-center justify-center rounded-full border bg-background"
                            >
                                <theme.icon class="h-6 w-6" />
                            </div>
                            <div class="text-center">
                                <div class="font-medium">{theme.label}</div>
                                <div class="text-xs text-muted-foreground">
                                    {theme.description}
                                </div>
                            </div>
                            {#if selectedTheme === theme.value}
                                <div class="absolute right-2 top-2">
                                    <Badge variant="default" class="text-xs">Active</Badge>
                                </div>
                            {/if}
                        </button>
                    {/each}
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Custom Themes -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <Palette class="h-5 w-5" />
                    Custom Themes
                </Card.Title>
                <Card.Description>Personalize your color scheme</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="flex items-center justify-between rounded-lg border p-4">
                    <div class="space-y-0.5">
                        <div class="text-base font-medium">Custom Color Schemes</div>
                        <div class="text-sm text-muted-foreground">
                            Create and manage your own custom themes
                        </div>
                    </div>
                    <Badge variant="secondary" class="text-xs">Coming Soon</Badge>
                </div>

                <div class="flex items-center justify-between rounded-lg border p-4">
                    <div class="space-y-0.5">
                        <div class="text-base font-medium">Advanced Theming</div>
                        <div class="text-sm text-muted-foreground">
                            Fine-tune colors, spacing, and typography
                        </div>
                    </div>
                    <Badge variant="secondary" class="text-xs">Coming Soon</Badge>
                </div>

                <div class="flex items-center justify-between rounded-lg border p-4">
                    <div class="space-y-0.5">
                        <div class="text-base font-medium">Theme Marketplace</div>
                        <div class="text-sm text-muted-foreground">
                            Browse and install community themes
                        </div>
                    </div>
                    <Badge variant="secondary" class="text-xs">Coming Soon</Badge>
                </div>
            </Card.Content>
        </Card.Root>
    </div>
</div>
