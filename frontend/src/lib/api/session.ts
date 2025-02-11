import type { Session, ShallowSession } from "$lib/models/session";
import type { PageResponse } from "$lib/models/page";

const sampleResponse: PageResponse<Session[]> = {
    pageInfo: {
        page: 1,
        pageSize: 10,
    },
    data: [
        {
            id: 'sdafopihv',
            title: 'Setting up the workspace',
            participants: [
                {
                    id: 'sadlkjrfhycnsf',
                    name: 'norpie',
                    user: true,
                    avatar: 'https://avatars.githubusercontent.com/u/46564751?v=4'
                },
                {
                    id: 'vasudfniunyave',
                    name: 'Mirabel',
                    user: false
                }
            ],
            chat: {
                messages: [
                    {
                        timestamp: '2021-09-01T12:00:00Z',
                        participant: "sadlkjrfhycnsf",
                        message: "Let\'s fix issue #5",
                    },
                    {
                        timestamp: '2021-09-01T12:01:00Z',
                        participant: "vasudfniunyave",
                        message: "I'm on it.",
                    },
                    {
                        timestamp: '2021-09-01T12:02:00Z',
                        participant: 'vasudfniunyave',
                        message: "I've generated a plan. Do you want to approve it?",
                    },
                    {
                        timestamp: '2021-09-01T12:03:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Yes, please.",
                    }
                ],

            }
        }
    ]
};

export async function fetchSession(sessionId: string): Promise<Session> {
    setTimeout(() => { }, 1000);
    return sampleResponse.data[0];
}

export async function fetchAllSessions(workspaceId: string, page: Page): Promise<PageResponse<ShallowSession[]>> {
    setTimeout(() => { }, 1000);
    return sampleResponse;
}
