import type { PageLoad } from './$types';
import { fetchAllSessions } from '$lib/api/session';
import { getWorkspaceById, sessions } from '$lib/store';

import { load as loadRoot } from "../../+page";

import { toast } from 'svelte-sonner';
import { goto } from '$app/navigation';

export async function load({ params }: PageLoad) {
    await loadRoot({ params });
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
    return {
        id: params.workspace_id,
        sessions: sessions.data,
        workspace: workspace,
    };
};
