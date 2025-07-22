export const ssr = false;

import type { LayoutLoad } from './$types';
import { get } from '$lib/request';
import type Result from '$lib/models/result';
import type { PageResponse } from '$lib/models/page';
import type { Workspace } from '$lib/models/workspace';
import { error } from '@sveltejs/kit';
import type { User } from '$lib/models/user';
import { authStore } from '$lib/auth/store.svelte';

export const load: LayoutLoad<{
    user: User | null;
    workspaces: Workspace[];
}> = async ({ fetch }) => {
    // Initialize auth store
    await authStore.initialize();

    // If not authenticated, return null user and empty workspaces
    if (!authStore.isAuthenticated || !authStore.user) {
        return {
            user: null,
            workspaces: []
        };
    }

    // User is authenticated, fetch workspaces
    try {
        const workspaceResult = await get<Result<PageResponse<Workspace[]>>>(
            `v1/me/workspace`,
            {
                page: 1,
                size: 10
            },
            fetch
        );

        if (!workspaceResult) {
            error(503, 'Unable to fetch workspaces');
        }

        if (!workspaceResult.data && workspaceResult.error) {
            console.error('Failed to load workspaces:', workspaceResult.error);
            return {
                user: authStore.user,
                workspaces: []
            };
        }

        if (!workspaceResult.data || !workspaceResult.data.data) {
            return {
                user: authStore.user,
                workspaces: []
            };
        }

        return {
            user: authStore.user,
            workspaces: workspaceResult.data.data
        };
    } catch (err) {
        console.error('Error loading workspaces:', err);
        return {
            user: authStore.user,
            workspaces: []
        };
    }
};
