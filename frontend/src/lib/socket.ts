import type { SessionEvent, SessionContent, MessageContent, AgentActionContent, AgentPromptContent, UserPromptResponseContent, AgentNewTaskEvent, AgentTaskEvent, AgentSpecUpdateEvent, AgentTerminalContentEvent, AcknowledgmentContent } from "./models/event";
import { toast } from 'svelte-sonner';
import { goto } from "$app/navigation";

// Type mapping from event type strings to their corresponding content types
type EventTypeMap = {
    'AcknowledgmentContent': AcknowledgmentContent;
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
    private authRetryCount = 0;
    private maxAuthRetries = 1;

    handlers: { [T in EventType]?: ((event: SessionEvent) => void)[] } = {};

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

    private async refreshToken(): Promise<boolean> {
        try {
            const response = await fetch('/api/v1/auth/refresh', {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
            });

            if (response.status === 401) {
                return false;
            }

            const result = await response.json();
            if (result.data?.access_token) {
                localStorage.setItem("accessToken", result.data.access_token);
                return true;
            }
            return false;
        } catch (error) {
            console.error("Token refresh failed:", error);
            return false;
        }
    }

    private async handleAuthFailure(): Promise<boolean> {
        if (this.authRetryCount >= this.maxAuthRetries) {
            goto("/login", {
                invalidateAll: true,
            });
            return false;
        }

        this.authRetryCount++;
        const refreshSuccess = await this.refreshToken();

        if (refreshSuccess) {
            // Update URL with new token
            const url = new URL(this.url);
            const accessToken = localStorage.getItem("accessToken");
            if (accessToken) {
                url.searchParams.set("access_token", accessToken);
                this.url = url.toString();
            }
            return true;
        } else {
            goto("/login", {
                invalidateAll: true,
            });
            return false;
        }
    }

    private reconnect(): void {
        if (!this.shouldReconnect) return;

        this.isReconnecting = true;
        this.setState("connecting");

        this.reconnectTimeout = setTimeout(async () => {
            try {
                this.socket = new WebSocket(this.url);
                this.listen();
            } catch (error) {
                console.error("Reconnection failed:", error);
                this.reconnect();
            }
        }, 3000);
    }

    addHandler<T extends EventType>(event: T, handler: (event: SessionEvent) => void): void {
        this.handlers[event] ??= [];
        (this.handlers[event] as ((event: SessionEvent) => void)[]).push(handler);
    }

    setStateHandler(handler: (status: ConnectionStatus) => void): void {
        this.stateHandler = handler;
    }

    listen(): void {
        this.setState("connecting");

        this.socket.onopen = () => {
            this.authRetryCount = 0; // Reset auth retry count on successful connection
            this.setState("open");
        };

        this.socket.onclose = async (event) => {
            this.setState("closed");

            // Handle authentication errors (typically code 1002 or 1008 for auth failures)
            if (event.code === 1002 || event.code === 1008 || event.code === 4001) {
                const authSuccess = await this.handleAuthFailure();
                if (authSuccess) {
                    this.reconnect();
                }
                return;
            }

            if (event.code !== 1000) {
                this.reconnect();
            }
        };

        this.socket.onerror = async (error) => {
            console.error("WebSocket error:", error);
            this.setState("error");
        };

        this.socket.onmessage = (e: MessageEvent) => {
            const event: SessionEvent = JSON.parse(e.data);
            const eventType = event.content.type as EventType;

            this.handlers[eventType]?.forEach(handler => {
                try {
                    handler(event as any);
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
