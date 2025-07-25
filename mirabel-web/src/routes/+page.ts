import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageLoad = async ({ parent }) => {
    const { user } = await parent();

    // Redirect to login if not authenticated
    if (!user) {
        throw redirect(303, '/login');
    }

    return {};
};
