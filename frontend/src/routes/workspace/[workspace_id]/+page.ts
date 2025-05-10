import type { PageLoad } from './$types';
import { selectedSession } from '$lib/store';

export const load: PageLoad = async ({ params }) => {

    selectedSession.set(null);
    return {
        id: params.workspace_id
    };
};
