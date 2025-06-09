export interface SessionEvent {
    id: string;              // Unique identifier for the event
    source: string;          // Who is the source of the event (e.g., user, agent, system)
    timestamp: string;       // ISO 8601 timestamp of when the event occurred
    content: SessionContent ; // Content of the event, can vary based on type
}

export type SessionContent = 
    | (MessageContent & { type: 'MessageContent' })
    | (AgentActionContent & { type: 'AgentActionContent' })
    | (AgentPromptContent & { type: 'AgentPromptContent' })
    | (UserPromptResponseContent & { type: 'UserPromptResponseContent' })
    | (AgentNewTaskEvent & { type: 'AgentNewTaskEvent' })
    | (AgentTaskEvent & { type: 'AgentTaskEvent' })
    | (AgentSpecUpdateEvent & { type: 'AgentSpecUpdateEvent' })
    | (AgentTerminalContentEvent & { type: 'AgentTerminalContentEvent' });

export interface MessageContent {
    message: string;
}

export interface AgentActionContent {
    action: string;
    description?: string;
}

export interface AgentPromptContent {
    promptId: string;
    prompt: string;
    options: string[];
    allowOther?: boolean;
}

export interface UserPromptResponseContent {
    promptId: string;
    response: string;
}

export interface AgentNewTaskEvent {
    taskId: string;
    parentId: string;
    description: string;
}

export interface AgentTaskEvent {
    taskId: string;
    status: string;
}

export interface AgentSpecUpdateEvent {
    spec: string;
}

export interface AgentTerminalContentEvent {
    content: string;
}
