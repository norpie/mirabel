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
            title: 'Setting up the workspace'
        }
    ]};

export async function fetchChat(chatId: string): Promise<Chat> {
    setTimeout(() => {}, 1000);
    return sampleResponse.data[0];
}

export async function fetchAllChats(workspaceId: string, page: Page): Promise<PageResponse<Chat[]>> {
    setTimeout(() => {}, 1000);
    return sampleResponse;
}
