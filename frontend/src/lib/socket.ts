import type { SessionEvent, SessionContent, MessageContent, AgentActionContent, AgentPromptContent, UserPromptResponseContent, AgentNewTaskEvent, AgentTaskEvent, AgentSpecUpdateEvent, AgentTerminalContentEvent } from "./models/event";
import { toast } from 'svelte-sonner';

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
type ConnectionStatus = "open" | "closed" | "connecting" | "error";

export class SessionSocketHandler {
    id: string;
    socket: WebSocket;
    status: ConnectionStatus = "closed";
    private url: string;
    private shouldReconnect = true;
    private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
    private toastId?: string | number;
    public isReconnecting = false;
    private isLeavingPage = false;
    private hasConnectedBefore = false;

    handlers: { [K in EventType]?: ((data: EventTypeMap[K]) => void)[] } = {};

    constructor(
        url: string,
        private stateHandler: (status: ConnectionStatus) => void = (status) => console.log(`WebSocket state: ${status}`)
    ) {
        this.id = crypto.randomUUID();
        this.url = url;
        this.socket = new WebSocket(url);
        this.listen();
    }

    private setState(status: ConnectionStatus): void {
        this.status = status;
        this.stateHandler(status);

        if (this.isLeavingPage) return;

        if (status === 'open') {
            this.isReconnecting = false;
            if (this.toastId) {
                toast.dismiss(this.toastId);
                if (this.hasConnectedBefore) {
                    toast.success('Connection restored');
                }
                this.toastId = undefined;
            }
            this.hasConnectedBefore = true;
        } else if (!this.toastId) {
            this.isReconnecting = status === 'connecting';
            this.toastId = toast.error('Connection lost', {
                description: 'The connection to the server was lost.'
            });
        }
    }

    private reconnect(): void {
        if (!this.shouldReconnect) return;

        this.isReconnecting = true;
        this.setState("connecting");

        this.reconnectTimeout = setTimeout(() => {
            try {
                this.socket = new WebSocket(this.url);
                this.listen();
            } catch (error) {
                console.error("Reconnection failed:", error);
                this.reconnect();
            }
        }, 3000);
    }

    addHandler<T extends EventType>(event: T, handler: (data: EventTypeMap[T]) => void): void {
        this.handlers[event] ??= [];
        (this.handlers[event] as ((data: EventTypeMap[T]) => void)[]).push(handler);
    }

    listen(): void {
        this.setState("connecting");

        this.socket.onopen = () => this.setState("open");

        this.socket.onclose = (event) => {
            this.setState("closed");
            if (event.code !== 1000) this.reconnect();
        };

        this.socket.onerror = () => this.setState("error");

        this.socket.onmessage = (e: MessageEvent) => {
            const event: SessionEvent = JSON.parse(e.data);
            const eventType = event.content.type as EventType;

            this.handlers[eventType]?.forEach(handler => {
                try {
                    handler(event.content as any);
                } catch (error) {
                    console.error(`Error in handler for ${eventType}:`, error);
                }
            });
        };
    }

    close(): void {
        this.shouldReconnect = false;
        this.isLeavingPage = true;

        if (this.toastId) {
            toast.dismiss(this.toastId);
            this.toastId = undefined;
        }

        if (this.reconnectTimeout) {
            clearTimeout(this.reconnectTimeout);
            this.reconnectTimeout = null;
        }

        if (this.socket.readyState === WebSocket.OPEN) {
            this.socket.close(1000, "Normal closure");
        }

        this.handlers = {};
    }

    send(data: SessionEvent): void {
        if (this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(JSON.stringify(data));
        } else {
            console.warn("WebSocket is not open, cannot send data.");
        }
    }

    manualReconnect(): void {
        if (this.socket.readyState !== WebSocket.OPEN &&
            this.socket.readyState !== WebSocket.CONNECTING) {
            this.shouldReconnect = true;
            this.reconnect();
        }
    }

    isCurrentlyReconnecting(): boolean {
        return this.isReconnecting;
    }
}
