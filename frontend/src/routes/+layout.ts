export const ssr = false;

import type { LayoutLoad } from './$types';
import { get } from '$lib/request';
import type Result from '$lib/models/result';
import type { PageResponse } from '$lib/models/page';
import type { Workspace } from '$lib/models/workspace';
import { error } from '@sveltejs/kit';
import type { User } from '$lib/models/user';

export const load: LayoutLoad<{
	user: User;
	workspaces: Workspace[];
}> = async ({ fetch }) => {
	const userResult = await get<Result<User>>('v1/me', fetch);
	if (!userResult) {
		error(503, 'Unable to fetch user data');
	}
	if (!userResult.data && userResult.error) {
        return {
            user: null,
            workspaces: []
        }
	}
	if (!userResult.data) {
		error(404, 'User not found');
	}
	const workspaceResult = await get<Result<PageResponse<Workspace[]>>>(
		`v1/me/workspace`,
		{
			page: 1,
			size: 10
		},
		fetch
	);
	if (!workspaceResult) {
		error(503, 'Unable to fetch workspaces');
	}
	if (!workspaceResult.data && workspaceResult.error) {
		error(500, `Failed to load workspaces: ${workspaceResult.error}`);
	}
	if (!workspaceResult.data || !workspaceResult.data.data) {
		error(404, 'Workspaces not found');
	}
	return {
		user: userResult.data,
		workspaces: workspaceResult.data.data
	};
}
