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
	import { Github, Chrome, Key, Webhook, Plus, Trash2, Eye, EyeOff, CheckCircle, AlertTriangle, ExternalLink } from 'lucide-svelte';

	// Mock integration data - replace with actual store/API calls
	let oauthConnections = $state([
		{
			id: 'github-123',
			provider: 'github',
			connectedAt: '2024-07-10T15:30:00Z',
			lastUsed: '2024-07-15T09:20:00Z',
			scopes: ['repo', 'user:email']
		}
	]);

	let apiKeys = $state([
		{
			id: 'key-1',
			name: 'Development API Key',
			keyPreview: 'mk_live_****1234',
			createdAt: '2024-07-01T10:00:00Z',
			lastUsed: '2024-07-16T08:45:00Z'
		}
	]);

	let webhooks = $state([
		{
			id: 'webhook-1',
			name: 'Slack Notifications',
			url: 'https://hooks.slack.com/services/****',
			events: ['session.completed', 'workspace.updated'],
			active: true,
			createdAt: '2024-06-15T12:00:00Z'
		}
	]);

	let showNewApiKeyDialog = $state(false);
	let showNewWebhookDialog = $state(false);
	let newApiKeyName = $state('');
	let newWebhook = $state({
		name: '',
		url: '',
		events: [] as string[]
	});

	let isLoading = $state(false);
	let successMessage = $state('');
	let errorMessage = $state('');

	const availableEvents = [
		{ value: 'session.created', label: 'Session Created' },
		{ value: 'session.completed', label: 'Session Completed' },
		{ value: 'session.failed', label: 'Session Failed' },
		{ value: 'workspace.created', label: 'Workspace Created' },
		{ value: 'workspace.updated', label: 'Workspace Updated' },
		{ value: 'workspace.deleted', label: 'Workspace Deleted' }
	];

	async function connectOAuth(provider: 'github' | 'google') {
		isLoading = true;
		try {
			// TODO: Redirect to OAuth provider
			console.log('Connecting to', provider);
			
			// Mock successful connection
			const newConnection = {
				id: `${provider}-${Date.now()}`,
				provider,
				connectedAt: new Date().toISOString(),
				lastUsed: new Date().toISOString(),
				scopes: provider === 'github' ? ['repo', 'user:email'] : ['profile', 'email']
			};
			
			oauthConnections = [...oauthConnections, newConnection];
			successMessage = `Successfully connected to ${provider}!`;
			setTimeout(() => successMessage = '', 3000);
		} catch (error) {
			errorMessage = `Failed to connect to ${provider}.`;
			console.error('OAuth connection failed:', error);
		} finally {
			isLoading = false;
		}
	}

	async function disconnectOAuth(connectionId: string) {
		isLoading = true;
		try {
			// TODO: API call to disconnect OAuth
			console.log('Disconnecting', connectionId);
			await new Promise(resolve => setTimeout(resolve, 1000));
			
			oauthConnections = oauthConnections.filter(conn => conn.id !== connectionId);
			successMessage = 'Successfully disconnected!';
			setTimeout(() => successMessage = '', 3000);
		} catch (error) {
			errorMessage = 'Failed to disconnect.';
			console.error('OAuth disconnection failed:', error);
		} finally {
			isLoading = false;
		}
	}

	async function createApiKey() {
		if (!newApiKeyName.trim()) {
			errorMessage = 'Please enter a name for the API key.';
			return;
		}

		isLoading = true;
		try {
			// TODO: API call to create new key
			console.log('Creating API key:', newApiKeyName);
			await new Promise(resolve => setTimeout(resolve, 1000));
			
			const newKey = {
				id: `key-${Date.now()}`,
				name: newApiKeyName.trim(),
				keyPreview: `mk_live_****${Math.random().toString(36).substr(-4)}`,
				createdAt: new Date().toISOString(),
				lastUsed: null as string | null
			};
			
			apiKeys = [...apiKeys, newKey];
			showNewApiKeyDialog = false;
			newApiKeyName = '';
			successMessage = 'API key created successfully!';
			setTimeout(() => successMessage = '', 3000);
		} catch (error) {
			errorMessage = 'Failed to create API key.';
			console.error('API key creation failed:', error);
		} finally {
			isLoading = false;
		}
	}

	async function deleteApiKey(keyId: string) {
		isLoading = true;
		try {
			// TODO: API call to delete key
			console.log('Deleting API key:', keyId);
			await new Promise(resolve => setTimeout(resolve, 1000));
			
			apiKeys = apiKeys.filter(key => key.id !== keyId);
			successMessage = 'API key deleted successfully!';
			setTimeout(() => successMessage = '', 3000);
		} catch (error) {
			errorMessage = 'Failed to delete API key.';
			console.error('API key deletion failed:', error);
		} finally {
			isLoading = false;
		}
	}

	async function createWebhook() {
		if (!newWebhook.name.trim() || !newWebhook.url.trim()) {
			errorMessage = 'Please fill in all required fields.';
			return;
		}

		isLoading = true;
		try {
			// TODO: API call to create webhook
			console.log('Creating webhook:', newWebhook);
			await new Promise(resolve => setTimeout(resolve, 1000));
			
			const webhook = {
				id: `webhook-${Date.now()}`,
				name: newWebhook.name.trim(),
				url: newWebhook.url.trim(),
				events: [...newWebhook.events],
				active: true,
				createdAt: new Date().toISOString()
			};
			
			webhooks = [...webhooks, webhook];
			showNewWebhookDialog = false;
			newWebhook = { name: '', url: '', events: [] };
			successMessage = 'Webhook created successfully!';
			setTimeout(() => successMessage = '', 3000);
		} catch (error) {
			errorMessage = 'Failed to create webhook.';
			console.error('Webhook creation failed:', error);
		} finally {
			isLoading = false;
		}
	}

	async function deleteWebhook(webhookId: string) {
		isLoading = true;
		try {
			// TODO: API call to delete webhook
			console.log('Deleting webhook:', webhookId);
			await new Promise(resolve => setTimeout(resolve, 1000));
			
			webhooks = webhooks.filter(webhook => webhook.id !== webhookId);
			successMessage = 'Webhook deleted successfully!';
			setTimeout(() => successMessage = '', 3000);
		} catch (error) {
			errorMessage = 'Failed to delete webhook.';
			console.error('Webhook deletion failed:', error);
		} finally {
			isLoading = false;
		}
	}

	function formatDate(dateString: string | null) {
		if (!dateString) return 'Never';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function toggleWebhookEvent(event: string) {
		if (newWebhook.events.includes(event)) {
			newWebhook.events = newWebhook.events.filter(e => e !== event);
		} else {
			newWebhook.events = [...newWebhook.events, event];
		}
	}
</script>

<div class="flex h-full flex-col">
	<div class="flex-none space-y-1 px-6 pt-2 pb-4">
		<h2 class="text-2xl font-bold tracking-tight">Integrations</h2>
		<p class="text-muted-foreground">
			Connect external services and manage API access.
		</p>
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

			<!-- OAuth Connections -->
			<Card.Root>
				<Card.Header>
					<Card.Title class="flex items-center gap-2">
						<ExternalLink class="h-5 w-5" />
						OAuth Connections
					</Card.Title>
					<Card.Description>
						Connect your GitHub and Google accounts for enhanced functionality
					</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<!-- GitHub -->
					<div class="flex items-center justify-between rounded-lg border p-4">
						<div class="flex items-center gap-3">
							<Github class="h-8 w-8" />
							<div>
								<div class="font-medium">GitHub</div>
								<div class="text-sm text-muted-foreground">
									Access repositories and commit data
								</div>
							</div>
						</div>
						<div class="flex items-center gap-2">
							{#if oauthConnections.find(conn => conn.provider === 'github')}
								<Badge variant="default">Connected</Badge>
								<Button 
									variant="outline" 
									size="sm"
									onclick={() => {
										const conn = oauthConnections.find(c => c.provider === 'github');
										if (conn) disconnectOAuth(conn.id);
									}}
									disabled={isLoading}
								>
									Disconnect
								</Button>
							{:else}
								<Button 
									size="sm"
									onclick={() => connectOAuth('github')}
									disabled={isLoading}
								>
									Connect
								</Button>
							{/if}
						</div>
					</div>

					<!-- Google -->
					<div class="flex items-center justify-between rounded-lg border p-4">
						<div class="flex items-center gap-3">
							<Chrome class="h-8 w-8" />
							<div>
								<div class="font-medium">Google</div>
								<div class="text-sm text-muted-foreground">
									Access Google services and data
								</div>
							</div>
						</div>
						<div class="flex items-center gap-2">
							{#if oauthConnections.find(conn => conn.provider === 'google')}
								<Badge variant="default">Connected</Badge>
								<Button 
									variant="outline" 
									size="sm"
									onclick={() => {
										const conn = oauthConnections.find(c => c.provider === 'google');
										if (conn) disconnectOAuth(conn.id);
									}}
									disabled={isLoading}
								>
									Disconnect
								</Button>
							{:else}
								<Button 
									size="sm"
									onclick={() => connectOAuth('google')}
									disabled={isLoading}
								>
									Connect
								</Button>
							{/if}
						</div>
					</div>

					{#if oauthConnections.length > 0}
						<div class="space-y-2">
							<h4 class="text-sm font-medium">Connected Accounts</h4>
							{#each oauthConnections as connection}
								<div class="text-xs text-muted-foreground">
									{connection.provider} - Connected {formatDate(connection.connectedAt)}, Last used {formatDate(connection.lastUsed)}
								</div>
							{/each}
						</div>
					{/if}
				</Card.Content>
			</Card.Root>

			<!-- API Keys -->
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="flex items-center gap-2">
								<Key class="h-5 w-5" />
								API Keys
							</Card.Title>
							<Card.Description>
								Manage API keys for programmatic access
							</Card.Description>
						</div>
						<Button size="sm" onclick={() => showNewApiKeyDialog = true}>
							<Plus class="h-4 w-4 mr-2" />
							New Key
						</Button>
					</div>
				</Card.Header>
				<Card.Content class="space-y-4">
					{#if apiKeys.length === 0}
						<div class="text-center py-8 text-muted-foreground">
							No API keys created yet. Create your first API key to get started.
						</div>
					{:else}
						<div class="space-y-3">
							{#each apiKeys as apiKey}
								<div class="flex items-center justify-between rounded-lg border p-4">
									<div class="space-y-1">
										<div class="font-medium">{apiKey.name}</div>
										<div class="text-sm text-muted-foreground font-mono">
											{apiKey.keyPreview}
										</div>
										<div class="text-xs text-muted-foreground">
											Created {formatDate(apiKey.createdAt)} • Last used {formatDate(apiKey.lastUsed)}
										</div>
									</div>
									<Button 
										variant="outline" 
										size="sm"
										onclick={() => deleteApiKey(apiKey.id)}
										disabled={isLoading}
									>
										<Trash2 class="h-4 w-4" />
									</Button>
								</div>
							{/each}
						</div>
					{/if}
				</Card.Content>
			</Card.Root>

			<!-- Webhooks -->
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="flex items-center gap-2">
								<Webhook class="h-5 w-5" />
								Webhooks
							</Card.Title>
							<Card.Description>
								Configure webhook endpoints for real-time notifications
							</Card.Description>
						</div>
						<Button size="sm" onclick={() => showNewWebhookDialog = true}>
							<Plus class="h-4 w-4 mr-2" />
							New Webhook
						</Button>
					</div>
				</Card.Header>
				<Card.Content class="space-y-4">
					{#if webhooks.length === 0}
						<div class="text-center py-8 text-muted-foreground">
							No webhooks configured yet. Create your first webhook to receive real-time notifications.
						</div>
					{:else}
						<div class="space-y-3">
							{#each webhooks as webhook}
								<div class="rounded-lg border p-4">
									<div class="flex items-start justify-between">
										<div class="space-y-1 flex-1">
											<div class="flex items-center gap-2">
												<div class="font-medium">{webhook.name}</div>
												<Badge variant={webhook.active ? 'default' : 'secondary'}>
													{webhook.active ? 'Active' : 'Inactive'}
												</Badge>
											</div>
											<div class="text-sm text-muted-foreground font-mono">
												{webhook.url}
											</div>
											<div class="text-xs text-muted-foreground">
												Events: {webhook.events.join(', ')}
											</div>
											<div class="text-xs text-muted-foreground">
												Created {formatDate(webhook.createdAt)}
											</div>
										</div>
										<Button 
											variant="outline" 
											size="sm"
											onclick={() => deleteWebhook(webhook.id)}
											disabled={isLoading}
										>
											<Trash2 class="h-4 w-4" />
										</Button>
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		</div>
</div>

<!-- New API Key Dialog -->
<Dialog.Root bind:open={showNewApiKeyDialog}>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>Create New API Key</Dialog.Title>
			<Dialog.Description>
				API keys provide programmatic access to your account.
			</Dialog.Description>
		</Dialog.Header>
		
		<div class="space-y-4">
			<div class="space-y-2">
				<Label for="api-key-name">Name</Label>
				<Input
					id="api-key-name"
					bind:value={newApiKeyName}
					placeholder="e.g., Production API Key"
				/>
			</div>
		</div>

		<Dialog.Footer>
			<Button 
				variant="outline" 
				onclick={() => {
					showNewApiKeyDialog = false;
					newApiKeyName = '';
				}}
			>
				Cancel
			</Button>
			<Button 
				onclick={createApiKey}
				disabled={!newApiKeyName.trim() || isLoading}
			>
				{isLoading ? 'Creating...' : 'Create Key'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<!-- New Webhook Dialog -->
<Dialog.Root bind:open={showNewWebhookDialog}>
	<Dialog.Content class="sm:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>Create New Webhook</Dialog.Title>
			<Dialog.Description>
				Webhooks send HTTP requests to your endpoint when events occur.
			</Dialog.Description>
		</Dialog.Header>
		
		<div class="space-y-4">
			<div class="space-y-2">
				<Label for="webhook-name">Name</Label>
				<Input
					id="webhook-name"
					bind:value={newWebhook.name}
					placeholder="e.g., Slack Notifications"
				/>
			</div>

			<div class="space-y-2">
				<Label for="webhook-url">Endpoint URL</Label>
				<Input
					id="webhook-url"
					bind:value={newWebhook.url}
					placeholder="https://your-endpoint.com/webhook"
				/>
			</div>

			<div class="space-y-2">
				<Label>Events</Label>
				<div class="grid grid-cols-1 gap-2">
					{#each availableEvents as event}
						<label class="flex items-center space-x-2 text-sm">
							<input
								type="checkbox"
								checked={newWebhook.events.includes(event.value)}
								onchange={() => toggleWebhookEvent(event.value)}
								class="h-4 w-4 rounded border-border"
							/>
							<span>{event.label}</span>
						</label>
					{/each}
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button 
				variant="outline" 
				onclick={() => {
					showNewWebhookDialog = false;
					newWebhook = { name: '', url: '', events: [] };
				}}
			>
				Cancel
			</Button>
			<Button 
				onclick={createWebhook}
				disabled={!newWebhook.name.trim() || !newWebhook.url.trim() || isLoading}
			>
				{isLoading ? 'Creating...' : 'Create Webhook'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
