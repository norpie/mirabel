import type { PageLoad } from './$types';

import { load as loadRoot } from "../+page";

export async function load({ params }: PageLoad) {
    await loadRoot({ params });
    return {};
}
