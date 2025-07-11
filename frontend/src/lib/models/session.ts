import type { PageResponse } from "./page";

export interface ShallowSession {
    id: string;
    title: string;
    avatar?: string;
}

export interface Session {
    id: string;
    title: string;
    archived?: boolean;
    timeline: PageResponse<TimelineEntry[]>;
    spec?: string;
    shell?: string[];
}

export function emptySession() {
    return {
        id: '',
        title: '',
        archived: false,
        timeline: {
            pageInfo: {
                page: 1,
                size: 20,
                total: 0,
            },
            data: [],
        },
        spec: undefined,
        shell: []
    }
}

export interface UserInteraction {
    type: 'message' | 'promptResponse';
    content?: string;
    promptId?: string;
    response?: string;
}

export interface TimelineEntry {
    id: string
    sessionId: string;
    content: TimelineEntryContent;
    contentType: 'message' | 'acknowledgment' | 'agentStatus' | 'prompt' | 'promptResponse' | 'action' | 'spec' | 'shell';
    createdAt: string;
}

export type TimelineEntryContent = MessageContent | AcknowledgmentContent | AgentStatusContent | Prompt | PromptResponse | ActionContent | SpecContent | ShellContent;

export interface MessageContent {
    type: 'message';
    sender: 'user' | 'agent';
    message: string;
}

export interface AcknowledgmentContent {
    type: 'acknowledgment';
    ackType: 'sent' | 'delivered' | 'seen';
}

export interface AgentStatusContent {
    type: 'agentStatus';
    status: 'thinking' | 'typing' | 'paused' | 'error';
}

export interface Prompt {
    type: 'prompt';
    promptId: string;
    options: string[];
}

export interface PromptResponse {
    type: 'promptResponse';
    promptId: string;
    response: string;
}

export interface ActionContent {
    type: 'action';
    actionType: 'command' | 'newFile' | 'editFile';
    message: string;
}

export interface SpecContent {
    type: 'spec';
    content: string;
}

export interface ShellContent {
    type: 'shell';
    lines: string[];
}

// export interface Plan {
//     goal: string;
//     spec: string;
//     children: PlanItem[];
// }
//
// export interface PlanItem {
//     id: string;
//     name: string;
//     description: string;
//     status: 'done' | 'paused' | 'in-progress' | 'todo';
//     children: PlanItem[];
// }
