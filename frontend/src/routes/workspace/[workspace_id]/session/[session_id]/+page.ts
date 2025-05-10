import type { PageLoad } from './$types';
import { fetchSession } from '$lib/api/session';

export const load: PageLoad = async ({ params }) => {
    const session = await fetchSession(params.session_id)
    return { session };
};
