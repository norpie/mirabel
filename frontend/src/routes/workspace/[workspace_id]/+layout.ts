import type { LayoutLoad } from './$types';

import { error } from '@sveltejs/kit';
import { get } from '$lib/request';
import type { Workspace } from '$lib/models/workspace';
import type Result from '$lib/models/result';
import type { ShallowSession } from '$lib/models/session';
import type { User } from '$lib/models/user';

export const load: LayoutLoad<{
    user: User;
    workspaces: Workspace[];
    workspaceId: string;
    sessions: ShallowSession[];
    workspace: Workspace;
}> = async ({ params, fetch, parent }) => {
    const parentData = await parent();
	const page = { page: 1, size: 10 };
    const workspace = await get<Result<Workspace>>(`v1/workspace/${params.workspace_id}`, fetch);
    if (!workspace) {
        error(503, 'Could not connect to the server');
    }
    if (!workspace.data && workspace.error) {
        error(500, `Failed to load workspace: ${workspace.error}`);
    }
    if (!workspace.data) {
        error(404, 'Workspace not found');
    }
	let sessions = await get(`v1/workspace/${params.workspace_id}/sessions`, page, fetch)
    if (!sessions) {
        error(503, 'Could not connect to the server');
    }
    if (!sessions.data && sessions.error) {
        error(500, `Failed to load sessions: ${sessions.error}`);
    }
    if (!sessions.data || !sessions.data.data) {
        error(404, 'No sessions found for this workspace');
    }
    return {
        ...parentData,
        workspaceId: params.workspace_id,
        sessions: sessions.data.data,
        workspace: workspace.data,
    };
};
