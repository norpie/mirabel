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
                        timestamp: '2021-08-31T12:00:00Z',
                        participant: "sadlkjrfhycnsf",
                        message: "Let\'s fix issue #5",
                    },
                    {
                        timestamp: '2021-09-31T12:01:00Z',
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
                    },
                    {
                        timestamp: '2021-09-01T12:04:00Z',
                        participant: 'vasudfniunyave',
                        message: "Great, I'll start working on it right away."
                    },
                    {
                        timestamp: '2021-09-01T12:05:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Thanks. Let me know if you need any help."
                    },
                    {
                        timestamp: '2021-09-01T12:06:00Z',
                        participant: 'vasudfniunyave',
                        message: "Will do! I'll keep you updated on my progress."
                    },
                    {
                        timestamp: '2021-09-01T12:07:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Sounds good. Take your time."
                    },
                    {
                        timestamp: '2021-09-01T12:08:00Z',
                        participant: 'vasudfniunyave',
                        message: "I've made some progress and am running into a couple of issues."
                    },
                    {
                        timestamp: '2021-09-01T12:09:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Can you tell me more about the issues?"
                    },
                    {
                        timestamp: '2021-09-01T12:10:00Z',
                        participant: 'vasudfniunyave',
                        message: "Sure, I'll explain the issues in detail."
                    },
                    {
                        timestamp: '2021-09-01T12:11:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Great, I'm ready to help."
                    },
                    {
                        timestamp: '2021-09-01T12:12:00Z',
                        participant: 'vasudfniunyave',
                        message: "First, the API call is not returning the expected data."
                    },
                    {
                        timestamp: '2021-09-01T12:13:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Can you check the API endpoint and parameters?"
                    },
                    {
                        timestamp: '2021-09-01T12:14:00Z',
                        participant: 'vasudfniunyave',
                        message: "I did, everything looks correct. Could it be a backend issue?"
                    },
                    {
                        timestamp: '2021-09-01T12:15:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Let's check with the backend team."
                    },
                    {
                        timestamp: '2021-09-01T12:16:00Z',
                        participant: 'vasudfniunyave',
                        message: "Okay, I'll wait for their response."
                    },
                    {
                        timestamp: '2021-09-01T12:17:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Great, thanks."
                    },
                    {
                        timestamp: '2025-02-01T12:18:00Z',
                        participant: 'vasudfniunyave',
                        message: "Also, the UI is not rendering the chat messages as expected."
                    },
                    {
                        timestamp: '2025-02-11T12:19:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Check the component and ensure data binding is correct."
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
