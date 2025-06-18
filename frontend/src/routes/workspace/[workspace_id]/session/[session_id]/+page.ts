import type { PageLoad } from './$types';
import { fetchSession } from '$lib/api/session';

import { load as loadWorkspace } from "../../+page";
import { goto } from '$app/navigation';

export async function load({params}: PageLoad) {
    let workspaceLoad = await loadWorkspace({params});
    if (!workspaceLoad) {
        goto("/not-found");
    }
    const session = await fetchSession(params.session_id)
    if (!session) {
        goto("/not-found");
    }
    return {
        id: params.session_id,
        session,
        sessions: workspaceLoad.sessions,
        workspace: workspaceLoad.workspace,
    };
};
