import type { PageLoad } from './$types';
import { fetchSession } from '$lib/api/session';

import { load as loadWorkspace } from "../../+page";
import { goto } from '$app/navigation';
import { toast } from 'svelte-sonner';

export async function load({params}: PageLoad) {
    let workspaceLoad = await loadWorkspace({params});
    console.log('load session page');
    if (!workspaceLoad) {
        goto("/not-found");
    }
    const session = await fetchSession(params.workspace_id, params.session_id)
    if (!session) {
        goto("/not-found");
    }
    if (!session.data) {
        toast.error(`Failed to load sessions: ${session.error}`);
        return;
    }
    return {
        id: params.session_id,
        workspace: workspaceLoad.workspace,
        sessions: workspaceLoad.sessions,
        session: session.data,
    };
};
