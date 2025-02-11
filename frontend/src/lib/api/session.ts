import type { Session } from "$lib/models/session";
import type { PageResponse } from "$lib/models/page";

const sampleResponse: PageResponse<Session[]> ={
    pageInfo: {
        page: 1,
        pageSize: 10,
    },
    data: [
        {
            id: 'sdafopihv',
            title: 'Setting up the workspace'
        }
    ]};

export async function fetchSession(sessionId: string): Promise<Session> {
    setTimeout(() => {}, 1000);
    return sampleResponse.data[0];
}

export async function fetchAllSessions(workspaceId: string, page: Page): Promise<PageResponse<Session[]>> {
    setTimeout(() => {}, 1000);
    return sampleResponse;
}
