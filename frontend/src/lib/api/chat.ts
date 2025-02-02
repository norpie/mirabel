import type { Chat } from "$lib/models/chat";
import type { PageResponse } from "$lib/models/page";

const sampleResponse: PageResponse<Chat[]> ={
    pageInfo: {
        page: 1,
        pageSize: 10,
    },
    data: [
        {
            id: 'sdafopihv',
            title: 'Setting up the repository'
        }
    ]};


export async function fetchAllChats(projectId: string, page: Page): Promise<PageResponse<Chat[]>> {
    setTimeout(() => {}, 1000);
    return sampleResponse;
}
