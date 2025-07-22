import type { PageLoad } from './$types';

import { get } from '$lib/request';
import type { ShallowSession } from '$lib/models/session';
import type { Workspace } from '$lib/models/workspace';
import { error } from '@sveltejs/kit';

export async function load({ params, fetch, parent }: PageLoad): Promise<{
    workspace_id: string;
    workspace: Workspace;
    sessions: ShallowSession[];
}> {
    const parentData = await parent();
    return {
        ...parentData,
        workspace_id: params.workspace_id
    };
}
