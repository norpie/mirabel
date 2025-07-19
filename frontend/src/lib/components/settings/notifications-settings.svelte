<script lang="ts">
    import * as Card from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Label } from '$lib/components/ui/label';
    import { Separator } from '$lib/components/ui/separator';
    import { Badge } from '$lib/components/ui/badge';
    import { Switch } from '$lib/components/ui/switch';
    import {
        Bell,
        Mail,
        MessageSquare,
        AlertTriangle,
        CheckCircle,
        Settings,
        Users
    } from 'lucide-svelte';

    // Mock notification settings - replace with actual store/API calls
    let notificationSettings = $state({
        email: {
            workspaceInvites: true,
            sessionUpdates: false,
            weeklyDigest: true,
            securityAlerts: true,
            marketing: false
        },
        push: {
            sessionComplete: true,
            mentions: true,
            workspaceActivity: false,
            systemMaintenance: true
        },
        inApp: {
            allNotifications: true,
            soundEnabled: false,
            desktopNotifications: true
        }
    });

    let isLoading = $state(false);
    let successMessage = $state('');

    async function saveSettings() {
        isLoading = true;
        try {
            // TODO: API call to save notification preferences
            console.log('Saving notification settings:', notificationSettings);
            await new Promise((resolve) => setTimeout(resolve, 1000)); // Mock delay

            successMessage = 'Notification preferences saved successfully!';
            setTimeout(() => (successMessage = ''), 3000);
        } catch (error) {
            console.error('Failed to save notification settings:', error);
        } finally {
            isLoading = false;
        }
    }

    function resetToDefaults() {
        notificationSettings = {
            email: {
                workspaceInvites: true,
                sessionUpdates: false,
                weeklyDigest: true,
                securityAlerts: true,
                marketing: false
            },
            push: {
                sessionComplete: true,
                mentions: true,
                workspaceActivity: false,
                systemMaintenance: true
            },
            inApp: {
                allNotifications: true,
                soundEnabled: false,
                desktopNotifications: true
            }
        };
    }

    // Check if push notifications are supported
    const pushSupported = $derived(typeof window !== 'undefined' && 'Notification' in window);
</script>

<div class="flex h-full flex-col">
    <div class="flex-none space-y-1 px-6 pb-4 pt-2">
        <h2 class="text-2xl font-bold tracking-tight">Notification Settings</h2>
        <p class="text-muted-foreground">Manage how and when you receive notifications.</p>
    </div>

    <Separator class="flex-none" />

    <div class="flex-1 space-y-6 p-6">
        {#if successMessage}
            <Alert.Root class="border-green-200 bg-green-50 text-green-800">
                <CheckCircle class="h-4 w-4" />
                <Alert.Title>Success</Alert.Title>
                <Alert.Description>{successMessage}</Alert.Description>
            </Alert.Root>
        {/if}

        {#if !pushSupported}
            <Alert.Root variant="destructive">
                <AlertTriangle class="h-4 w-4" />
                <Alert.Title>Push Notifications Not Supported</Alert.Title>
                <Alert.Description>
                    Your browser doesn't support push notifications. Some notification features may
                    not work.
                </Alert.Description>
            </Alert.Root>
        {/if}

        <!-- Email Notifications -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <Mail class="h-5 w-5" />
                    Email Notifications
                </Card.Title>
                <Card.Description>
                    Choose which email notifications you'd like to receive
                </Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="space-y-4">
                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Workspace Invites</div>
                            <div class="text-sm text-muted-foreground">
                                When you're invited to join a workspace
                            </div>
                        </div>
                        <Switch bind:checked={notificationSettings.email.workspaceInvites} />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Session Updates</div>
                            <div class="text-sm text-muted-foreground">
                                When sessions you're involved in are updated
                            </div>
                        </div>
                        <Switch bind:checked={notificationSettings.email.sessionUpdates} />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Weekly Digest</div>
                            <div class="text-sm text-muted-foreground">
                                Weekly summary of your workspace activity
                            </div>
                        </div>
                        <Switch bind:checked={notificationSettings.email.weeklyDigest} />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="flex items-center gap-2 text-base font-medium">
                                Security Alerts
                                <Badge variant="outline" class="text-xs">Required</Badge>
                            </div>
                            <div class="text-sm text-muted-foreground">
                                Important security notifications for your account
                            </div>
                        </div>
                        <Switch
                            checked={notificationSettings.email.securityAlerts}
                            disabled={true}
                        />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Marketing & Product Updates</div>
                            <div class="text-sm text-muted-foreground">
                                Information about new features and product updates
                            </div>
                        </div>
                        <Switch bind:checked={notificationSettings.email.marketing} />
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Push Notifications -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <Bell class="h-5 w-5" />
                    Push Notifications
                </Card.Title>
                <Card.Description>Real-time notifications sent to your device</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="space-y-4">
                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Session Complete</div>
                            <div class="text-sm text-muted-foreground">
                                When a session you're monitoring completes
                            </div>
                        </div>
                        <Switch
                            bind:checked={notificationSettings.push.sessionComplete}
                            disabled={!pushSupported}
                        />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Mentions & Comments</div>
                            <div class="text-sm text-muted-foreground">
                                When someone mentions you or comments on your content
                            </div>
                        </div>
                        <Switch
                            bind:checked={notificationSettings.push.mentions}
                            disabled={!pushSupported}
                        />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Workspace Activity</div>
                            <div class="text-sm text-muted-foreground">
                                General workspace updates and activity
                            </div>
                        </div>
                        <Switch
                            bind:checked={notificationSettings.push.workspaceActivity}
                            disabled={!pushSupported}
                        />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">System Maintenance</div>
                            <div class="text-sm text-muted-foreground">
                                Scheduled maintenance and system updates
                            </div>
                        </div>
                        <Switch
                            bind:checked={notificationSettings.push.systemMaintenance}
                            disabled={!pushSupported}
                        />
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- In-App Notifications -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <MessageSquare class="h-5 w-5" />
                    In-App Notifications
                </Card.Title>
                <Card.Description>Notifications shown while using the application</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="space-y-4">
                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">All Notifications</div>
                            <div class="text-sm text-muted-foreground">
                                Show notifications within the app interface
                            </div>
                        </div>
                        <Switch bind:checked={notificationSettings.inApp.allNotifications} />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Sound Effects</div>
                            <div class="text-sm text-muted-foreground">
                                Play sound when notifications appear
                            </div>
                        </div>
                        <Switch
                            bind:checked={notificationSettings.inApp.soundEnabled}
                            disabled={!notificationSettings.inApp.allNotifications}
                        />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Desktop Notifications</div>
                            <div class="text-sm text-muted-foreground">
                                Show notifications even when the app is in the background
                            </div>
                        </div>
                        <Switch
                            bind:checked={notificationSettings.inApp.desktopNotifications}
                            disabled={!notificationSettings.inApp.allNotifications ||
                                !pushSupported}
                        />
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Actions -->
        <div class="flex justify-between pt-4">
            <Button variant="outline" onclick={resetToDefaults}>Reset to Defaults</Button>
            <div class="flex gap-2">
                <Button variant="outline" onclick={() => window.location.reload()}>Cancel</Button>
                <Button onclick={saveSettings} disabled={isLoading}>
                    {isLoading ? 'Saving...' : 'Save Changes'}
                </Button>
            </div>
        </div>
    </div>
</div>
