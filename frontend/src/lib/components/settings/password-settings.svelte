<script lang="ts">
    import * as Card from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import { Separator } from '$lib/components/ui/separator';
    import * as Alert from '$lib/components/ui/alert';
    import { Lock, Shield, AlertTriangle, CheckCircle } from 'lucide-svelte';

    let passwordForm = $state({
        currentPassword: '',
        newPassword: '',
        confirmPassword: ''
    });

    let isLoading = $state(false);
    let showPasswords = $state(false);
    let validationErrors = $state<string[]>([]);
    let successMessage = $state('');

    $effect(() => {
        // Reset success message when form changes
        if (
            passwordForm.currentPassword ||
            passwordForm.newPassword ||
            passwordForm.confirmPassword
        ) {
            successMessage = '';
        }
    });

    function validatePassword(password: string): string[] {
        const errors: string[] = [];
        if (password.length < 8) errors.push('At least 8 characters');
        if (!/[A-Z]/.test(password)) errors.push('One uppercase letter');
        if (!/[a-z]/.test(password)) errors.push('One lowercase letter');
        if (!/\d/.test(password)) errors.push('One number');
        if (!/[!@#$%^&*(),.?":{}|<>]/.test(password)) errors.push('One special character');
        return errors;
    }

    function getPasswordStrength(password: string): {
        score: number;
        label: string;
        color: string;
    } {
        const errors = validatePassword(password);
        const score = Math.max(0, 5 - errors.length);

        if (score <= 1) return { score, label: 'Very Weak', color: 'bg-red-500' };
        if (score <= 2) return { score, label: 'Weak', color: 'bg-orange-500' };
        if (score <= 3) return { score, label: 'Fair', color: 'bg-yellow-500' };
        if (score <= 4) return { score, label: 'Good', color: 'bg-blue-500' };
        return { score, label: 'Strong', color: 'bg-green-500' };
    }

    $effect(() => {
        // Validate passwords in real-time
        validationErrors = [];

        if (passwordForm.newPassword) {
            const errors = validatePassword(passwordForm.newPassword);
            if (errors.length > 0) {
                validationErrors = errors;
            }
        }

        if (
            passwordForm.confirmPassword &&
            passwordForm.newPassword !== passwordForm.confirmPassword
        ) {
            validationErrors = [...validationErrors, 'Passwords do not match'];
        }
    });

    async function handlePasswordChange() {
        if (
            !passwordForm.currentPassword ||
            !passwordForm.newPassword ||
            !passwordForm.confirmPassword
        ) {
            validationErrors = ['All fields are required'];
            return;
        }

        if (validationErrors.length > 0) return;

        isLoading = true;
        try {
            // TODO: API call to change password
            console.log('Changing password...');
            await new Promise((resolve) => setTimeout(resolve, 2000)); // Mock delay

            // Success
            successMessage = 'Password changed successfully!';
            passwordForm = {
                currentPassword: '',
                newPassword: '',
                confirmPassword: ''
            };
        } catch (error) {
            validationErrors = ['Failed to change password. Please check your current password.'];
            console.error('Password change failed:', error);
        } finally {
            isLoading = false;
        }
    }

    const strengthInfo = $derived(getPasswordStrength(passwordForm.newPassword));
</script>

<div class="flex h-full flex-col">
    <div class="flex-none space-y-1 px-6 pb-4 pt-2">
        <h2 class="text-2xl font-bold tracking-tight">Change Password</h2>
        <p class="text-muted-foreground">Update your password to keep your account secure.</p>
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

        {#if validationErrors.length > 0}
            <Alert.Root variant="destructive">
                <AlertTriangle class="h-4 w-4" />
                <Alert.Title>Password Requirements</Alert.Title>
                <Alert.Description>
                    <ul class="mt-2 space-y-1">
                        {#each validationErrors as error}
                            <li class="flex items-center gap-2">
                                <span class="h-1 w-1 rounded-full bg-current"></span>
                                {error}
                            </li>
                        {/each}
                    </ul>
                </Alert.Description>
            </Alert.Root>
        {/if}

        <!-- Password Change Form -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <Lock class="h-5 w-5" />
                    Change Password
                </Card.Title>
                <Card.Description>
                    Enter your current password and choose a new one
                </Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="space-y-2">
                    <Label for="current-password">Current Password</Label>
                    <Input
                        id="current-password"
                        type={showPasswords ? 'text' : 'password'}
                        bind:value={passwordForm.currentPassword}
                        placeholder="Enter current password"
                    />
                </div>

                <div class="space-y-2">
                    <Label for="new-password">New Password</Label>
                    <Input
                        id="new-password"
                        type={showPasswords ? 'text' : 'password'}
                        bind:value={passwordForm.newPassword}
                        placeholder="Enter new password"
                    />
                    {#if passwordForm.newPassword}
                        <div class="space-y-2">
                            <div class="flex items-center justify-between text-sm">
                                <span class="text-muted-foreground">Password strength:</span>
                                <Badge
                                    variant={strengthInfo.score >= 4 ? 'default' : 'secondary'}
                                    class="text-xs"
                                >
                                    {strengthInfo.label}
                                </Badge>
                            </div>
                            <div class="h-2 w-full rounded-full bg-muted">
                                <div
                                    class="h-full rounded-full transition-all {strengthInfo.color}"
                                    style="width: {(strengthInfo.score / 5) * 100}%"
                                ></div>
                            </div>
                        </div>
                    {/if}
                </div>

                <div class="space-y-2">
                    <Label for="confirm-password">Confirm New Password</Label>
                    <Input
                        id="confirm-password"
                        type={showPasswords ? 'text' : 'password'}
                        bind:value={passwordForm.confirmPassword}
                        placeholder="Confirm new password"
                    />
                </div>

                <div class="flex items-center space-x-2">
                    <input
                        id="show-passwords"
                        type="checkbox"
                        bind:checked={showPasswords}
                        class="h-4 w-4 rounded border-border"
                    />
                    <Label for="show-passwords" class="text-sm font-normal">Show passwords</Label>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Password Requirements -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    <Shield class="h-5 w-5" />
                    Password Requirements
                </Card.Title>
            </Card.Header>
            <Card.Content>
                <ul class="space-y-2 text-sm text-muted-foreground">
                    <li class="flex items-center gap-2">
                        <CheckCircle class="h-4 w-4 text-green-500" />
                        At least 8 characters long
                    </li>
                    <li class="flex items-center gap-2">
                        <CheckCircle class="h-4 w-4 text-green-500" />
                        Contains uppercase and lowercase letters
                    </li>
                    <li class="flex items-center gap-2">
                        <CheckCircle class="h-4 w-4 text-green-500" />
                        Includes at least one number
                    </li>
                    <li class="flex items-center gap-2">
                        <CheckCircle class="h-4 w-4 text-green-500" />
                        Contains at least one special character
                    </li>
                </ul>
            </Card.Content>
        </Card.Root>

        <!-- Actions -->
        <div class="flex justify-end space-x-2 pt-4">
            <Button
                variant="outline"
                onclick={() => {
                    passwordForm = { currentPassword: '', newPassword: '', confirmPassword: '' };
                    validationErrors = [];
                    successMessage = '';
                }}
            >
                Cancel
            </Button>
            <Button
                onclick={handlePasswordChange}
                disabled={isLoading ||
                    validationErrors.length > 0 ||
                    !passwordForm.currentPassword ||
                    !passwordForm.newPassword ||
                    !passwordForm.confirmPassword}
            >
                {isLoading ? 'Changing Password...' : 'Change Password'}
            </Button>
        </div>
    </div>
</div>
