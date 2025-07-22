<script lang="ts">
    import * as Card from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import { Separator } from '$lib/components/ui/separator';
    import { Badge } from '$lib/components/ui/badge';
    import { Switch } from '$lib/components/ui/switch';
    import * as Alert from '$lib/components/ui/alert';
    import * as Dialog from '$lib/components/ui/dialog';
    import {
        User,
        Download,
        Trash2,
        AlertTriangle,
        CheckCircle,
        Shield,
        Calendar
    } from 'lucide-svelte';

    // Mock user data - replace with actual store/API calls
    let userAccount = $state({
        id: '123',
        username: 'johndoe',
        email: 'john@example.com',
        createdAt: '2024-01-15T10:30:00Z',
        lastLogin: '2024-07-16T09:15:00Z',
        totalWorkspaces: 3,
        totalSessions: 42
    });

    let emailPreferences = $state({
        receiveUpdates: true,
        securityNotifications: true,
        marketingEmails: false
    });

    let isLoading = $state(false);
    let showDeleteDialog = $state(false);
    let deleteConfirmation = $state('');
    let successMessage = $state('');
    let errorMessage = $state('');

    async function saveEmailPreferences() {
        isLoading = true;
        try {
            // TODO: API call to save email preferences
            console.log('Saving email preferences:', emailPreferences);
            await new Promise((resolve) => setTimeout(resolve, 1000));

            successMessage = 'Email preferences updated successfully!';
            setTimeout(() => (successMessage = ''), 3000);
        } catch (error) {
            errorMessage = 'Failed to update email preferences.';
            console.error('Failed to save email preferences:', error);
        } finally {
            isLoading = false;
        }
    }

    async function exportAccountData() {
        isLoading = true;
        try {
            // TODO: API call to export account data
            console.log('Exporting account data...');
            await new Promise((resolve) => setTimeout(resolve, 2000));

            // Mock data export
            const data = {
                account: userAccount,
                preferences: emailPreferences,
                exportedAt: new Date().toISOString()
            };

            const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'mirabel-account-data.json';
            a.click();
            URL.revokeObjectURL(url);

            successMessage = 'Account data exported successfully!';
            setTimeout(() => (successMessage = ''), 3000);
        } catch (error) {
            errorMessage = 'Failed to export account data.';
            console.error('Failed to export account data:', error);
        } finally {
            isLoading = false;
        }
    }

    async function deleteAccount() {
        if (deleteConfirmation !== 'DELETE') {
            errorMessage = 'Please type DELETE to confirm account deletion.';
            return;
        }

        isLoading = true;
        try {
            // TODO: API call to delete account
            console.log('Deleting account...');
            await new Promise((resolve) => setTimeout(resolve, 2000));

            // Redirect to login or goodbye page
            window.location.href = '/login';
        } catch (error) {
            errorMessage = 'Failed to delete account. Please try again.';
            console.error('Failed to delete account:', error);
        } finally {
            isLoading = false;
            showDeleteDialog = false;
        }
    }

    function formatDate(dateString: string) {
        return new Date(dateString).toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        });
    }

    function formatRelativeTime(dateString: string) {
        const date = new Date(dateString);
        const now = new Date();
        const diffInDays = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24));

        if (diffInDays === 0) return 'Today';
        if (diffInDays === 1) return 'Yesterday';
        if (diffInDays < 30) return `${diffInDays} days ago`;
        if (diffInDays < 365) return `${Math.floor(diffInDays / 30)} months ago`;
        return `${Math.floor(diffInDays / 365)} years ago`;
    }

    // Clear messages when actions change
    $effect(() => {
        if (deleteConfirmation) {
            errorMessage = '';
        }
    });
</script>

<div class="flex h-full flex-col">
    <div class="flex-none space-y-1 px-6 pb-4 pt-2">
        <h2 class="text-2xl font-bold tracking-tight">Account Settings</h2>
        <p class="text-muted-foreground">Manage your account preferences and data.</p>
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

        {#if errorMessage}
            <Alert.Root variant="destructive">
                <AlertTriangle class="h-4 w-4" />
                <Alert.Title>Error</Alert.Title>
                <Alert.Description>{errorMessage}</Alert.Description>
            </Alert.Root>
        {/if}

        <!-- Account Overview -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <User class="h-5 w-5" />
                    Account Overview
                </Card.Title>
                <Card.Description>Your account information and statistics</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <div class="space-y-2">
                        <Label>Account Status</Label>
                        <div class="flex items-center gap-2">
                            <Badge variant="default">Active</Badge>
                            <Badge variant="outline">Verified</Badge>
                        </div>
                    </div>
                    <div class="space-y-2">
                        <Label>Member Since</Label>
                        <div class="text-sm">
                            {formatDate(userAccount.createdAt)}
                        </div>
                    </div>
                    <div class="space-y-2">
                        <Label>Last Login</Label>
                        <div class="text-sm">
                            {formatRelativeTime(userAccount.lastLogin)}
                        </div>
                    </div>
                    <div class="space-y-2">
                        <Label>Account ID</Label>
                        <div class="font-mono text-sm text-muted-foreground">
                            {userAccount.id}
                        </div>
                    </div>
                </div>

                <Separator />

                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <div class="space-y-2">
                        <Label>Total Workspaces</Label>
                        <div class="text-2xl font-bold">
                            {userAccount.totalWorkspaces}
                        </div>
                    </div>
                    <div class="space-y-2">
                        <Label>Total Sessions</Label>
                        <div class="text-2xl font-bold">
                            {userAccount.totalSessions}
                        </div>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Email Preferences -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <Shield class="h-5 w-5" />
                    Email Preferences
                </Card.Title>
                <Card.Description>Control what emails you receive from us</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="space-y-4">
                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Product Updates</div>
                            <div class="text-sm text-muted-foreground">
                                Get notified about new features and improvements
                            </div>
                        </div>
                        <Switch bind:checked={emailPreferences.receiveUpdates} />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="flex items-center gap-2 text-base font-medium">
                                Security Notifications
                                <Badge variant="outline" class="text-xs">Required</Badge>
                            </div>
                            <div class="text-sm text-muted-foreground">
                                Important security alerts for your account
                            </div>
                        </div>
                        <Switch checked={emailPreferences.securityNotifications} disabled={true} />
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="space-y-0.5">
                            <div class="text-base font-medium">Marketing Emails</div>
                            <div class="text-sm text-muted-foreground">
                                Tips, best practices, and promotional content
                            </div>
                        </div>
                        <Switch bind:checked={emailPreferences.marketingEmails} />
                    </div>
                </div>

                <div class="flex justify-end">
                    <Button onclick={saveEmailPreferences} disabled={isLoading}>
                        {isLoading ? 'Saving...' : 'Save Preferences'}
                    </Button>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Data Export -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <Download class="h-5 w-5" />
                    Data Export
                </Card.Title>
                <Card.Description>Download a copy of your account data</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="space-y-2">
                    <p class="text-sm text-muted-foreground">
                        Export your account information, preferences, and associated data. This
                        includes your profile, workspaces, and session history.
                    </p>
                    <p class="text-sm text-muted-foreground">
                        The export will be provided as a JSON file that you can download
                        immediately.
                    </p>
                </div>

                <Button onclick={exportAccountData} disabled={isLoading} variant="outline">
                    {isLoading ? 'Exporting...' : 'Export Account Data'}
                </Button>
            </Card.Content>
        </Card.Root>

        <!-- Danger Zone -->
        <Card.Root class="border-red-200">
            <Card.Header>
                <Card.Title class="flex items-center gap-2 text-red-600">
                    <Trash2 class="h-5 w-5" />
                    Danger Zone
                </Card.Title>
                <Card.Description>Irreversible and destructive actions</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <Alert.Root variant="destructive">
                    <AlertTriangle class="h-4 w-4" />
                    <Alert.Title>Account Deletion</Alert.Title>
                    <Alert.Description>
                        Once you delete your account, there is no going back. This will permanently
                        delete your profile, workspaces, and all associated data.
                    </Alert.Description>
                </Alert.Root>

                <Button variant="destructive" onclick={() => (showDeleteDialog = true)}>
                    Delete Account
                </Button>
            </Card.Content>
        </Card.Root>
    </div>
</div>

<!-- Delete Account Dialog -->
<Dialog.Root bind:open={showDeleteDialog}>
    <Dialog.Content class="sm:max-w-md">
        <Dialog.Header>
            <Dialog.Title class="flex items-center gap-2 text-red-600">
                <Trash2 class="h-5 w-5" />
                Delete Account
            </Dialog.Title>
            <Dialog.Description>
                This action cannot be undone. This will permanently delete your account and all
                associated data.
            </Dialog.Description>
        </Dialog.Header>

        <div class="space-y-4">
            <Alert.Root variant="destructive">
                <AlertTriangle class="h-4 w-4" />
                <Alert.Title>This will delete:</Alert.Title>
                <Alert.Description>
                    <ul class="mt-2 space-y-1 text-sm">
                        <li>• Your profile and account information</li>
                        <li>• All workspaces you own ({userAccount.totalWorkspaces} workspaces)</li>
                        <li>• All your sessions and data ({userAccount.totalSessions} sessions)</li>
                        <li>• Your notification and preference settings</li>
                    </ul>
                </Alert.Description>
            </Alert.Root>

            <div class="space-y-2">
                <Label for="delete-confirmation">
                    Type <strong>DELETE</strong> to confirm:
                </Label>
                <Input
                    id="delete-confirmation"
                    bind:value={deleteConfirmation}
                    placeholder="Type DELETE here"
                />
            </div>
        </div>

        <Dialog.Footer>
            <Button
                variant="outline"
                onclick={() => {
                    showDeleteDialog = false;
                    deleteConfirmation = '';
                }}
            >
                Cancel
            </Button>
            <Button
                variant="destructive"
                onclick={deleteAccount}
                disabled={deleteConfirmation !== 'DELETE' || isLoading}
            >
                {isLoading ? 'Deleting...' : 'Delete Account'}
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
