import type { SessionEvent, SessionContent, MessageContent, AgentActionContent, AgentPromptContent, UserPromptResponseContent, AgentNewTaskEvent, AgentTaskEvent, AgentSpecUpdateEvent, AgentTerminalContentEvent } from "./models/event";

// Type mapping from event type strings to their corresponding content types
type EventTypeMap = {
    'MessageContent': MessageContent;
    'AgentActionContent': AgentActionContent;
    'AgentPromptContent': AgentPromptContent;
    'UserPromptResponseContent': UserPromptResponseContent;
    'AgentNewTaskEvent': AgentNewTaskEvent;
    'AgentTaskEvent': AgentTaskEvent;
    'AgentSpecUpdateEvent': AgentSpecUpdateEvent;
    'AgentTerminalContentEvent': AgentTerminalContentEvent;
}

// Valid event type keys
type EventType = keyof EventTypeMap;

export class SessionSocketHandler {
    socket: WebSocket;
    stateHandler: (status: "open" | "closed" | "error") => void;
    handlers: { [K in EventType]?: ((data: EventTypeMap[K]) => void)[] } = {};
    private url: string;

    constructor(socket: WebSocket, stateHandler?: (status: "open" | "closed" | "error") => void) {
        this.socket = socket;
        if (!stateHandler) {
            this.stateHandler = (status: string) => {console.log(`WebSocket state changed: ${status}`); };
        } else {
            this.stateHandler = stateHandler;
        }
        this.url = socket.url;
        this.listen();
    }

    private reconnect(): void {
        console.log("Attempting to reconnect in 3 seconds...");
        setTimeout(() => {
            try {
                this.socket = new WebSocket(this.url);
                this.listen();
            } catch (error) {
                console.error("Reconnection failed:", error);
                this.reconnect(); // Try again if failed
            }
        }, 3000);
    }

    addHandler<T extends EventType>(event: T, handler: (data: EventTypeMap[T]) => void): void {
        if (!this.handlers[event]) {
            this.handlers[event] = [];
        }
        (this.handlers[event] as ((data: EventTypeMap[T]) => void)[]).push(handler);
    }

    listen(): void {
        this.socket.onopen = () => {
            this.stateHandler("open");
        };
        this.socket.onclose = () => {
            this.stateHandler("closed");
        };
        this.socket.onerror = (error: Event) => {
            this.stateHandler("error");
            console.error("WebSocket error:", error);
            this.reconnect();
        };
        this.socket.onmessage = (e: MessageEvent) => {
            const event: SessionEvent = JSON.parse(e.data);
            const eventType = event.content.type as EventType;
            const handlers = this.handlers[eventType];
            if (handlers) {
                for (const handler of handlers) {
                    try {
                        handler(event.content as any);
                    } catch (error) {
                        console.error("Error in handler for event", eventType, ":", error);
                    }
                }
            }
        };
    }

    close(): void {
        if (this.socket.readyState === WebSocket.OPEN) {
            this.socket.close();
        } else {
            console.warn("WebSocket is not open, cannot close.");
        }
    }

    send(data: SessionEvent): void {
        if (this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(JSON.stringify(data));
        } else {
            console.warn("WebSocket is not open, cannot send data.");
        }
    }
}
