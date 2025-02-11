import type { Page, PageResponse } from '$lib/models/page';
import type { Workspace } from '$lib/models/workspace';

import { generateId } from '$lib/utils';

const sampleResponse: PageResponse<Workspace[]> = {
    pageInfo: {
        page: 1,
        pageSize: 10,
    },
    data: [
        {
            id: generateId(),
            name: 'Mirabel',
        },
        {
            id: generateId(),
            name: 'Finanalize',
        },
        {
            id: generateId(),
            name: 'Analytical',
        },
        {
            id: generateId(),
            name: 'Alice',
        },
        {
            id: generateId(),
            name: 'µLLM-API',
        },
        {
            id: generateId(),
            name: 'Random Local Workspace',
        }
    ]
};

export async function fetchAllWorkspaces(page: Page): Promise<PageResponse<Workspace[]>> {
    setTimeout(() => { }, 1000);
    return sampleResponse;
}

export async function fetchRecentWorkspace(): Promise<Workspace> {
    setTimeout(() => { }, 1000);
    return sampleResponse.data[0];
}
