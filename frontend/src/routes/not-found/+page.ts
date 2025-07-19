import type { PageLoad } from './$types';

export async function load({ params, parent }: PageLoad) {
    let parentData = await parent();
    return {
        ...parentData
    };
}
