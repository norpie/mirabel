import type { PageLoad } from './$types';
import { fetchChat } from '$lib/api/chat';

export const load: PageLoad = async ({ params }) => {
    const chat = await fetchChat(params.id)
    return { chat };
};
