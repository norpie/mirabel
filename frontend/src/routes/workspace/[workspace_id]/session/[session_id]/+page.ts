import type { PageLoad } from './$types';

import { connectWebSocket, get } from '$lib/request';
import type { Session, ShallowSession, TimelineEntry, UserInteraction } from '$lib/models/session';
import type { Workspace } from '$lib/models/workspace';
import { error } from '@sveltejs/kit';
import type { SocketHandler } from '$lib/socket.svelte';
import type Result from '$lib/models/result';

export async function load({params, fetch, parent}: PageLoad): Promise<
{
    workspace_id: string;
    workspace: Workspace;
    sessions: ShallowSession[];
    session_id: string;
    session: Session;
    socket: SocketHandler<TimelineEntry, UserInteraction>;
}>{
    const session: Result<Session> = await get(`v1/workspace/${params.workspace_id}/session/${params.session_id}`, undefined, fetch);
    if (!session) {
        error(503, 'Could not connect to the server');
    }
    if (!session.data && session.error) {
        error(500, `Failed to load session: ${session.error}`);
    }
    if (!session.data) {
        error(404, 'Session not found');
    }
    const socket = connectWebSocket(`v1/workspace/${params.workspace_id}/session/${params.session_id}/socket`, undefined);
    const parentData = await parent();
    return {
        ...parentData,
        workspace_id: params.workspace_id,
        session_id: params.session_id,
        session: session.data,
        socket: socket,
    };
};
