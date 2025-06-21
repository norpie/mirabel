import type { PageLoad } from './$types';

import { connectWebSocket, get } from '$lib/request';
import type { Session, ShallowSession } from '$lib/models/session';
import type { Workspace } from '$lib/models/workspace';
import type { SessionSocketHandler } from '$lib/socket';
import { error } from '@sveltejs/kit';

export async function load({params, fetch}: PageLoad): Promise<
{
    workspace_id: string;
    workspace: Workspace;
    sessions: ShallowSession[];
    session_id: string;
    session: Session;
    socket: SessionSocketHandler;
}>{
    const parentData = await parent();
    const session = await get(`v1/workspace/${params.workspace_id}/sessions/${params.session_id}`, fetch);
    if (!session) {
        error(503, 'Could not connect to the server');
    }
    if (!session.data && session.error) {
        error(500, `Failed to load session: ${session.error}`);
    }
    if (!session.data) {
        error(404, 'Session not found');
    }
    const socket = connectWebSocket('v1/' + 'session/' + params.session_id, undefined);
    return {
        ...parentData,
        workspace_id: params.workspace_id,
        session_id: params.session_id,
        session: session.data,
        socket: socket,
    };
};
