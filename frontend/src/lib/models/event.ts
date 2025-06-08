export interface SessionEvent {
    id: string;              // Unique identifier for the event
    source: string;          // Who is the source of the event (e.g., user, agent, system)
    timestamp: string;       // ISO 8601 timestamp of when the event occurred
    type: string;            // Type of event (e.g., message, action, etc.)
    content: SessionContent; // Content of the event, can vary based on type
}

export interface MessageContent { // type: `message`
    message: string; // The actual message content
}

export interface AgentActionContent { // type: `action`
    action: string;       // Action taken by the agent (e.g. "run", "edit")
    description?: string; // Optional description of the action (e.g. "Running `find . -name '*.txt'`", "Committing changes", "Added `get_file` function")
}

export interface AgentPromptContent { // type: `prompt`
    promptId: string;     // Unique identifier for the prompt
    prompt: string;       // The prompt given to the agent (e.g. "Which branch should I use?")
    options: string[];    // The options available for the user to choose from (e.g. ["main", "develop", "Create new branch"])
    allowOther?: boolean; // Optional flag indicating if the user can provide an answer not in the options
}

export interface UserPromptResponseContent { // type: `prompt_response`
    promptId: string; // Unique identifier for the prompt
    response: string; // The user's response to the prompt (e.g. "main", "develop", "Create new branch", or other user input if allowOther is true)
}

export interface AgentNewTaskEvent { // type: `new_task`
    taskId: string;      // Unique identifier for the task
    parentId: string;    // Parent task ID for task hierarchy
    description: string; // Description of the task (e.g., "Run a command", "Edit a file")
}

export interface AgentTaskEvent { // type: `task_status`
    taskId: string; // Unique identifier for the task
    status: string; // Status of the task (e.g., "pending", "in_progress", "completed", "failed")
}

export interface AgentSpecUpdateEvent { // type: `spec_update`
    spec: string; // Content of the spec
}

export interface AgentTerminalContentEvent { // type: `terminal_content`
    content: string; // Content of the terminal update
}

export type SessionContent = MessageContent | AgentActionContent | AgentPromptContent | UserPromptResponseContent | AgentNewTaskEvent | AgentTaskEvent | AgentSpecUpdateEvent | AgentTerminalContentEvent;
