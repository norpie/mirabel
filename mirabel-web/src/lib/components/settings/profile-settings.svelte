<script lang="ts">
    import * as Card from '$lib/components/ui/card';
    import * as Avatar from '$lib/components/ui/avatar';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import { Separator } from '$lib/components/ui/separator';
    import { Upload, User } from 'lucide-svelte';

    // Mock user data - replace with actual store/API calls
    let userProfile = $state({
        id: '123',
        username: 'johndoe',
        email: 'john@example.com',
        createdAt: '2024-01-15T10:30:00Z',
        avatar: null as string | null
    });

    let isLoading = $state(false);

    async function handleSave() {
        isLoading = true;
        try {
            // TODO: API call to update profile
            console.log('Saving profile:', userProfile);
            await new Promise((resolve) => setTimeout(resolve, 1000)); // Mock delay
        } catch (error) {
            console.error('Failed to save profile:', error);
        } finally {
            isLoading = false;
        }
    }

    function handleAvatarUpload(event: Event) {
        const target = event.target as HTMLInputElement;
        const file = target.files?.[0];
        if (file) {
            // TODO: Upload avatar to server
            console.log('Uploading avatar:', file);
        }
    }

    function formatDate(dateString: string) {
        return new Date(dateString).toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    }
</script>

<div class="flex h-full flex-col">
    <div class="flex-none space-y-1 px-6 pb-4 pt-2">
        <h2 class="text-2xl font-bold tracking-tight">Profile Settings</h2>
        <p class="text-muted-foreground">Manage your profile information and avatar.</p>
    </div>

    <Separator class="flex-none" />

    <div class="flex-1 space-y-6 p-6">
        <!-- Avatar Section -->
        <Card.Root>
            <Card.Header>
                <Card.Title>Profile Picture</Card.Title>
                <Card.Description>Upload and manage your profile picture</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="flex items-center space-x-4">
                    <Avatar.Root class="h-20 w-20">
                        <Avatar.Image src={userProfile.avatar} alt="Profile" />
                        <Avatar.Fallback class="text-lg">
                            <User class="h-8 w-8" />
                        </Avatar.Fallback>
                    </Avatar.Root>
                    <div class="space-y-2">
                        <Label for="avatar-upload" class="cursor-pointer">
                            <Button variant="outline" size="sm" class="cursor-pointer">
                                <Upload class="mr-2 h-4 w-4" />
                                Upload Avatar
                            </Button>
                            <input
                                id="avatar-upload"
                                type="file"
                                accept="image/*"
                                class="sr-only"
                                onchange={handleAvatarUpload}
                            />
                        </Label>
                        <p class="text-xs text-muted-foreground">JPG, PNG or GIF. Max 2MB.</p>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Personal Information -->
        <Card.Root>
            <Card.Header>
                <Card.Title>Personal Information</Card.Title>
                <Card.Description>Update your username and email address</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <div class="space-y-2">
                        <Label for="username">Username</Label>
                        <Input
                            id="username"
                            type="text"
                            bind:value={userProfile.username}
                            placeholder="Enter username"
                        />
                    </div>
                    <div class="space-y-2">
                        <Label for="email">Email</Label>
                        <Input
                            id="email"
                            type="email"
                            bind:value={userProfile.email}
                            placeholder="Enter email"
                        />
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Account Information -->
        <Card.Root>
            <Card.Header>
                <Card.Title>Account Information</Card.Title>
                <Card.Description>Read-only account details</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <div class="space-y-2">
                        <Label>User ID</Label>
                        <Input value={userProfile.id} readonly class="bg-muted" />
                    </div>
                    <div class="space-y-2">
                        <Label>Account Created</Label>
                        <Input
                            value={formatDate(userProfile.createdAt)}
                            readonly
                            class="bg-muted"
                        />
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Actions -->
        <div class="flex justify-end space-x-2 pt-4">
            <Button variant="outline" onclick={() => window.location.reload()}>Cancel</Button>
            <Button onclick={handleSave} disabled={isLoading}>
                {isLoading ? 'Saving...' : 'Save Changes'}
            </Button>
        </div>
    </div>
</div>
