import type { PageLoad } from './$types';
import { fetchAllSessions } from '$lib/api/session';
import { getWorkspaceById, sessions } from '$lib/store';

import { load as loadLayout } from "../../+layout";

import { toast } from 'svelte-sonner';
import { goto } from '$app/navigation';

export async function load({ params }: PageLoad) {
    const parentData = await loadLayout();
    console.log('load workspace page');
	const page = { page: 1, size: 10 };
    const workspace = getWorkspaceById(params.workspace_id);
    if (!workspace) {
        goto("/not-found");
        return;
    }
	let sessions = await fetchAllSessions(params.workspace_id, page);
    if (!sessions) {
        goto("/not-found");
    }
    if (!sessions.data || !sessions.data.data) {
        toast.error(`Failed to load sessions: ${sessions.error}`);
        return;
    }
    return {
        id: params.workspace_id,
        sessions: sessions.data.data,
        workspace: workspace,
    };
};
