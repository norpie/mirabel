export const ssr = false;

import type { LayoutLoad } from './$types';
import { user, workspaces, selectedWorkspace, sessions } from '$lib/store';
import { get as getStore } from 'svelte/store';
import { fetchUser } from '$lib/api/user';
import { toast } from 'svelte-sonner';
import { get } from '$lib/request';
import type Result from '$lib/models/result';
import type { PageResponse } from '$lib/models/page';
import type { Workspace } from '$lib/models/workspace';

export const load: LayoutLoad = async () => {
	let uResult = await fetchUser();
	console.log(uResult);
	user.set(uResult.data);
	if (uResult.error) {
		toast.error(uResult.error);
		console.error(uResult);
		return;
	}

	let wResult = await get<Result<PageResponse<Workspace[]>>>(`v1/me/workspaces`, {
		page: 1,
		size: 10
	});
	if (wResult.error) {
		toast.error(wResult.error);
		console.error(wResult);
		workspaces.set([]);
	} else {
		workspaces.set(wResult.data.data);
	}

};
