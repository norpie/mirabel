export interface SessionEvent {
    id: string;              // Unique identifier for the event
    source: string;          // Who is the source of the event (e.g., user, agent, system)
    timestamp: string;       // ISO 8601 timestamp of when the event occurred
    content: SessionContent ; // Content of the event, can vary based on type
}

export type SessionContent =
    | AcknowledgmentContent
    | MessageContent
    | AgentActionContent
    | AgentPromptContent
    | UserPromptResponseContent
    | AgentNewTaskEvent
    | AgentTaskEvent
    | AgentSpecUpdateEvent
    | AgentTerminalContentEvent;

export interface AcknowledgmentContent {
    type: 'AcknowledgmentContent';
    ackType: 'delivered' | 'seen' | 'thinking' | 'typing' | 'error';
}

export interface MessageContent {
    type: 'MessageContent';
    authorId: string;
    message: string;
}

export interface AgentActionContent {
    type: 'AgentActionContent';
    action: string;
    description?: string;
}

export interface AgentPromptContent {
    type: 'AgentPromptContent';
    promptId: string;
    prompt: string;
    options: string[];
    allowOther?: boolean;
}

export interface UserPromptResponseContent {
    type: 'UserPromptResponseContent';
    promptId: string;
    response: string;
}

export interface AgentNewTaskEvent {
    type: 'AgentNewTaskEvent';
    taskId: string;
    parentId: string;
    description: string;
}

export interface AgentTaskEvent {
    type: 'AgentTaskEvent';
    taskId: string;
    status: string;
}

export interface AgentSpecUpdateEvent {
    type: 'AgentSpecUpdateEvent';
    spec: string;
}

export interface AgentTerminalContentEvent {
    type: 'AgentTerminalContentEvent';
    content: string;
}
