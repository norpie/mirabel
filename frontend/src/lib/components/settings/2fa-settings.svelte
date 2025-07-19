<script lang="ts">
    import * as Card from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import { Separator } from '$lib/components/ui/separator';
    import { Badge } from '$lib/components/ui/badge';
    import { Switch } from '$lib/components/ui/switch';
    import {
        Shield,
        ShieldCheck,
        Key,
        AlertTriangle,
        Copy,
        CheckCircle,
        QrCode
    } from 'lucide-svelte';

    // Mock user 2FA status - replace with actual store/API calls
    let twoFactorStatus = $state({
        enabled: false,
        setupComplete: false,
        backupCodes: [] as string[]
    });

    let setupStep = $state<'disabled' | 'setup' | 'verify' | 'complete'>('disabled');
    let qrCodeUrl = $state('');
    let verificationCode = $state('');
    let isLoading = $state(false);
    let errorMessage = $state('');
    let successMessage = $state('');

    $effect(() => {
        if (twoFactorStatus.enabled) {
            setupStep = 'complete';
        } else {
            setupStep = 'disabled';
        }
    });

    async function handleToggle2FA() {
        if (twoFactorStatus.enabled) {
            // Disable 2FA
            isLoading = true;
            try {
                // TODO: API call to disable 2FA
                console.log('Disabling 2FA...');
                await new Promise((resolve) => setTimeout(resolve, 1000));

                twoFactorStatus.enabled = false;
                twoFactorStatus.setupComplete = false;
                twoFactorStatus.backupCodes = [];
                setupStep = 'disabled';
                successMessage = 'Two-factor authentication has been disabled.';
            } catch (error) {
                errorMessage = 'Failed to disable 2FA. Please try again.';
                console.error('2FA disable failed:', error);
            } finally {
                isLoading = false;
            }
        } else {
            // Start 2FA setup
            await startSetup();
        }
    }

    async function startSetup() {
        isLoading = true;
        setupStep = 'setup';
        try {
            // TODO: API call to generate QR code
            console.log('Starting 2FA setup...');
            await new Promise((resolve) => setTimeout(resolve, 1000));

            // Mock QR code URL
            qrCodeUrl =
                'https://chart.googleapis.com/chart?chs=200x200&chld=M|0&cht=qr&chl=otpauth://totp/Mirabel:user@example.com?secret=JBSWY3DPEHPK3PXP&issuer=Mirabel';
            setupStep = 'verify';
        } catch (error) {
            errorMessage = 'Failed to start 2FA setup. Please try again.';
            console.error('2FA setup failed:', error);
            setupStep = 'disabled';
        } finally {
            isLoading = false;
        }
    }

    async function verifySetup() {
        if (!verificationCode || verificationCode.length !== 6) {
            errorMessage = 'Please enter a valid 6-digit code.';
            return;
        }

        isLoading = true;
        try {
            // TODO: API call to verify 2FA code
            console.log('Verifying 2FA code:', verificationCode);
            await new Promise((resolve) => setTimeout(resolve, 1000));

            // Mock successful verification
            twoFactorStatus.enabled = true;
            twoFactorStatus.setupComplete = true;
            twoFactorStatus.backupCodes = [
                'ABC123DEF456',
                'GHI789JKL012',
                'MNO345PQR678',
                'STU901VWX234',
                'YZA567BCD890'
            ];

            setupStep = 'complete';
            verificationCode = '';
            successMessage = 'Two-factor authentication has been successfully enabled!';
        } catch (error) {
            errorMessage = 'Invalid verification code. Please try again.';
            console.error('2FA verification failed:', error);
        } finally {
            isLoading = false;
        }
    }

    function copyBackupCode(code: string) {
        navigator.clipboard.writeText(code);
        // TODO: Show toast notification
        console.log('Copied backup code:', code);
    }

    function downloadBackupCodes() {
        const content = twoFactorStatus.backupCodes.join('\n');
        const blob = new Blob([content], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'mirabel-backup-codes.txt';
        a.click();
        URL.revokeObjectURL(url);
    }

    // Clear messages when setup step changes
    $effect(() => {
        errorMessage = '';
        successMessage = '';
    });
</script>

<div class="flex h-full flex-col">
    <div class="flex-none space-y-1 px-6 pb-4 pt-2">
        <h2 class="text-2xl font-bold tracking-tight">Two-Factor Authentication</h2>
        <p class="text-muted-foreground">Add an extra layer of security to your account.</p>
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

        <!-- 2FA Status -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="flex items-center gap-2">
                    {#if twoFactorStatus.enabled}
                        <ShieldCheck class="h-5 w-5 text-green-500" />
                    {:else}
                        <Shield class="h-5 w-5" />
                    {/if}
                    Two-Factor Authentication
                </Card.Title>
                <Card.Description>
                    {#if twoFactorStatus.enabled}
                        Your account is protected with two-factor authentication.
                    {:else}
                        Secure your account with an authenticator app.
                    {/if}
                </Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="flex items-center justify-between">
                    <div class="space-y-0.5">
                        <div class="text-base font-medium">
                            Status:
                            {#if twoFactorStatus.enabled}
                                <Badge variant="default" class="ml-1">Enabled</Badge>
                            {:else}
                                <Badge variant="secondary" class="ml-1">Disabled</Badge>
                            {/if}
                        </div>
                        <div class="text-sm text-muted-foreground">
                            {#if twoFactorStatus.enabled}
                                Two-factor authentication is active on your account
                            {:else}
                                Enable 2FA to add an extra layer of security
                            {/if}
                        </div>
                    </div>
                    <Switch
                        checked={twoFactorStatus.enabled}
                        onCheckedChange={handleToggle2FA}
                        disabled={isLoading}
                    />
                </div>
            </Card.Content>
        </Card.Root>

        {#if setupStep === 'setup' || setupStep === 'verify'}
            <!-- Setup Process -->
            <Card.Root>
                <Card.Header>
                    <Card.Title class="flex items-center gap-2">
                        <QrCode class="h-5 w-5" />
                        Setup Authenticator App
                    </Card.Title>
                    <Card.Description>
                        Scan the QR code with your authenticator app
                    </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    {#if setupStep === 'setup'}
                        <div class="text-center">
                            <div
                                class="inline-block flex h-32 w-32 items-center justify-center rounded border bg-muted"
                            >
                                Loading QR Code...
                            </div>
                        </div>
                    {:else if setupStep === 'verify'}
                        <div class="space-y-4">
                            <div class="text-center">
                                {#if qrCodeUrl}
                                    <img
                                        src={qrCodeUrl}
                                        alt="2FA QR Code"
                                        class="mx-auto h-40 w-40 rounded border"
                                    />
                                {/if}
                            </div>

                            <ol class="space-y-2 text-sm text-muted-foreground">
                                <li>
                                    1. Download an authenticator app like Google Authenticator or
                                    Authy
                                </li>
                                <li>2. Scan the QR code above with your authenticator app</li>
                                <li>3. Enter the 6-digit code from your app below</li>
                            </ol>

                            <div class="space-y-2">
                                <Label for="verification-code">Verification Code</Label>
                                <Input
                                    id="verification-code"
                                    type="text"
                                    bind:value={verificationCode}
                                    placeholder="Enter 6-digit code"
                                    maxlength={6}
                                    class="text-center text-lg tracking-widest"
                                />
                            </div>

                            <div class="flex gap-2">
                                <Button
                                    variant="outline"
                                    onclick={() => (setupStep = 'disabled')}
                                    disabled={isLoading}
                                >
                                    Cancel
                                </Button>
                                <Button
                                    onclick={verifySetup}
                                    disabled={isLoading || verificationCode.length !== 6}
                                    class="flex-1"
                                >
                                    {isLoading ? 'Verifying...' : 'Verify & Enable'}
                                </Button>
                            </div>
                        </div>
                    {/if}
                </Card.Content>
            </Card.Root>
        {/if}

        {#if twoFactorStatus.enabled && twoFactorStatus.backupCodes.length > 0}
            <!-- Backup Codes -->
            <Card.Root>
                <Card.Header>
                    <Card.Title class="flex items-center gap-2">
                        <Key class="h-5 w-5" />
                        Backup Codes
                    </Card.Title>
                    <Card.Description>
                        Save these backup codes in a safe place. You can use them to access your
                        account if you lose your authenticator device.
                    </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    <Alert.Root>
                        <AlertTriangle class="h-4 w-4" />
                        <Alert.Title>Important</Alert.Title>
                        <Alert.Description>
                            Each backup code can only be used once. Store them securely and don't
                            share them with anyone.
                        </Alert.Description>
                    </Alert.Root>

                    <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
                        {#each twoFactorStatus.backupCodes as code, index}
                            <div
                                class="flex items-center justify-between rounded border p-3 font-mono text-sm"
                            >
                                <span>{code}</span>
                                <Button
                                    variant="ghost"
                                    size="sm"
                                    onclick={() => copyBackupCode(code)}
                                >
                                    <Copy class="h-4 w-4" />
                                </Button>
                            </div>
                        {/each}
                    </div>

                    <Button variant="outline" onclick={downloadBackupCodes} class="w-full">
                        Download Backup Codes
                    </Button>
                </Card.Content>
            </Card.Root>
        {/if}
    </div>
</div>
