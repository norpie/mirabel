<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import SettingsSidebar from '$lib/components/settings-sidebar.svelte';
	import ProfileSettings from '$lib/components/settings/profile-settings.svelte';
	import TwoFactorSettings from '$lib/components/settings/2fa-settings.svelte';
	import type { Component, Snippet } from 'svelte';
	import AccountSettings from './settings/account-settings.svelte';
	import IntegrationsSettings from './settings/integrations-settings.svelte';
	import PasswordSettings from './settings/password-settings.svelte';
	
	import AppearanceSettings from './settings/appearance-settings.svelte';
	import NotificationsSettings from './settings/notifications-settings.svelte';

	// Default component to render if none provided
	function defaultComponent(value: string) {
		return '<p>' + value + '</p>';
	}

	let activeTab = $state('profile');
	let ActiveComponent = $derived.by(() => {
		const group = items.find((g) => g.items.some((i) => i.value === activeTab));
		const item = group?.items.find((i) => i.value === activeTab);
		return item?.component;
	});

	type TabItem = {
		value: string;
		title: string;
		component: any;
	};

	type TabGroup = {
		title: string;
		items: TabItem[];
	};

	let items: TabGroup[] = [
		{
			title: 'General',
			items: [
				{ value: 'profile', title: 'Profile', component: ProfileSettings },
				{ value: 'account', title: 'Account', component: AccountSettings },
				{ value: 'integrations', title: 'Integrations', component: IntegrationsSettings }
			]
		},
		{
			title: 'Security',
			items: [
				{ value: '2fa', title: 'Two-Factor Authentication', component: TwoFactorSettings },
                { value: 'password', title: 'Change Password', component: PasswordSettings }
			]
		},
		{
			title: 'Preferences',
			items: [
				{ value: 'appearance', title: 'Appearance', component: AppearanceSettings },
				{ value: 'notifications', title: 'Notifications', component: NotificationsSettings },
			]
		},
	];
</script>

<Sidebar.Provider class="h-full w-full">
	<SettingsSidebar bind:activeTab {items} class="h-full overflow-hidden" />
	<Sidebar.Inset class="h-full">
		<ActiveComponent />
	</Sidebar.Inset>
</Sidebar.Provider>
