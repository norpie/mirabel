import type { PageLoad } from './$types';
import { fetchSession } from '$lib/api/session';

import { load as loadWorkspace } from "../../+page";

export async function load({params}: PageLoad) {
    let workspaceLoad = await loadWorkspace({params});
    const session = await fetchSession(params.session_id)
    return {
        id: params.session_id,
        session,
        sessions: workspaceLoad.sessions,
        workspace: workspaceLoad.workspace,
    };
};
