import type { Page, PageResponse } from '$lib/models/page';
import type { Project } from '$lib/models/project';

import { generateId } from '$lib/utils';

import Github from 'lucide-svelte/icons/github';
import Gitlab from 'lucide-svelte/icons/gitlab';
import HardDrive from 'lucide-svelte/icons/hard-drive';
import FileQuestion from 'lucide-svelte/icons/file-question';

const sampleResponse: PageResponse<Project[]> = {
    pageInfo: {
        page: 1,
        pageSize: 10,
    },
    data: [
        {
            id: generateId(),
            name: 'Mirabel',
            url: 'https://github.com/norpie/mirabel',
            platform: 'GitHub',
            icon: Github
        },
        {
            id: generateId(),
            name: 'Finanalize',
            url: 'https://github.com/norpie/finanalize',
            platform: 'GitHub',
            icon: Github
        },
        {
            id: generateId(),
            name: 'Analytical',
            url: 'https://github.com/norpie/analytical',
            platform: 'GitHub',
            icon: Github
        },
        {
            id: generateId(),
            name: 'Alice',
            url: 'https://github.com/norpie/alice',
            platform: 'Gitlab',
            icon: Gitlab
        },
        {
            id: generateId(),
            name: 'µLLM-API',
            url: 'https://github.com/norpie/uLLM-API',
            platform: 'Bitbucket',
            icon: FileQuestion
        },
        {
            id: generateId(),
            name: 'Random Local Project',
            platform: 'Local',
            icon: HardDrive
        }
    ]
};

export async function fetchAllProjects(page: Page): Promise<PageResponse<Project[]>> {
    setTimeout(() => { }, 1000);
    return sampleResponse;
}

export async function fetchRecentProject(): Promise<Project> {
    setTimeout(() => { }, 1000);
    return sampleResponse.data[0];
}
