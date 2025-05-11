import type { PageLoad } from './$types';
import { fetchAllSessions } from '$lib/api/session';
import { getWorkspaceById, sessions } from '$lib/store';

import { load as loadRoot } from "../../+page";

import { toast } from 'svelte-sonner';

export async function load({ params }: PageLoad) {
    await loadRoot({ params });
    console.log('load workspace page');
	const page = { page: 1, size: 10 };
	let sessions = await fetchAllSessions(params.workspace_id, page);
    return {
        id: params.workspace_id,
        sessions: sessions.data,
        workspace: getWorkspaceById(params.workspace_id),
    };
};
