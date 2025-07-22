<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import * as Card from '$lib/components/ui/card/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Label } from '$lib/components/ui/label/index.js';
    import { superForm } from 'sveltekit-superforms';
    import { zodClient } from 'sveltekit-superforms/adapters';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    import { authStore } from '$lib/auth/store.svelte.js';
    import { loginSchema } from '$lib/auth/schemas.js';

    // Redirect if already logged in
    onMount(() => {
        if (authStore.isAuthenticated) {
            goto('/');
        }
    });

    const { form, errors, enhance, submitting } = superForm(
        { email: '', password: '' },
        {
            SPA: true,
            validators: zodClient(loginSchema),
            onSubmit: async ({ formData, cancel }) => {
                cancel();
                const email = formData.get('email') as string;
                const password = formData.get('password') as string;

                const result = await authStore.login({ email, password });
                if (!result.success && result.error) {
                    // Error handling is done in the store
                }
            }
        }
    );
</script>

<div class="flex h-screen w-full items-center justify-center px-4">
    <Card.Root class="mx-auto max-w-sm">
        <Card.Header>
            <Card.Title class="text-2xl">Login</Card.Title>
            <Card.Description>Enter your email below to login to your account</Card.Description>
        </Card.Header>
        <Card.Content>
            <form method="POST" use:enhance>
                <div class="grid gap-4">
                    <div class="grid gap-2">
                        <Label for="email">Email</Label>
                        <Input
                            id="email"
                            name="email"
                            type="email"
                            placeholder="me@example.com"
                            bind:value={$form.email}
                            disabled={$submitting || authStore.isLoading}
                        />
                        {#if $errors.email}
                            <p class="text-sm text-red-500">{$errors.email}</p>
                        {/if}
                    </div>

                    <div class="grid gap-2">
                        <Label for="password">Password</Label>
                        <Input
                            id="password"
                            name="password"
                            type="password"
                            bind:value={$form.password}
                            disabled={$submitting || authStore.isLoading}
                        />
                        {#if $errors.password}
                            <p class="text-sm text-red-500">{$errors.password}</p>
                        {/if}
                    </div>

                    <Button
                        type="submit"
                        class="w-full"
                        disabled={$submitting || authStore.isLoading}
                    >
                        {#if $submitting || authStore.isLoading}
                            Logging in...
                        {:else}
                            Login
                        {/if}
                    </Button>
                </div>
            </form>
            <div class="mt-4 text-center text-sm">
                Don't have an account?
                <a href="/register" class="underline"> Register </a>
            </div>
        </Card.Content>
    </Card.Root>
</div>
