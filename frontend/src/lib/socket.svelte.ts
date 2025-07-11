import { toast } from "svelte-sonner";

export class SocketHandler<T, U> {
    // Internal state
    private endpoint: string;
    private firstConnect: boolean = true;
    private socket: WebSocket | null = null;
    status: 'connecting' | 'open' | 'closing' | 'closed' | 'error' = $state("closed");

    // Message handling
    private messageHandler: ((message: T) => void) | null = null;

    // Reconnect settings
    private autoReconnect: boolean = true;
    private isReconnecting: boolean = false;
    private reconnectInterval: number = 5000;
    private reconnectTimer: ReturnType<typeof setTimeout> | null = null;

    // Toasting
    private connectionToastId: string | number | null = null;

    constructor(endpoint: string) {
        this.endpoint = endpoint;
    }

    public connect(): void {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
            this.genericErrorToast('Already Connected', 'You are already connected to the WebSocket server.');
            return;
        }
        try {
            this.status = 'connecting';
            this.socket = new WebSocket(this.endpoint);
            this.setHandlers();
        } catch (error) {
            this.status = 'error';
            this.genericErrorToast('Connection Error', 'Failed to connect to the WebSocket server: ' + (error instanceof Error ? error.message : 'Unknown error'));
            console.log(error);
        }
        this.isReconnecting = false;
    }

    public disconnect(): void {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
        if (this.connectionToastId) {
            toast.dismiss(this.connectionToastId);
            this.connectionToastId = null;
        }
        this.autoReconnect = false;
        this.socket?.close();
        this.socket = null;
        this.status = 'closed';
    }

    public manualReconnect(): void {
        this.autoReconnect = true;
        this.reconnect();
    }

    private reconnect(): void {
        if (!this.autoReconnect || this.isReconnecting) return;
        this.isReconnecting = true;
        try {
            this.connectionToast('Reconnecting...', 'Attempting to reconnect to the WebSocket server...', 'info');
            this.connect();
        } catch (error) {
            this.genericErrorToast('Reconnect Error', 'Failed to initiate reconnection: ' + (error instanceof Error ? error.message : 'Unknown error'));
        }
    }

    public setMessageHandler(handler: (message: T) => void): void {
        this.messageHandler = handler;
    }

    private setHandlers(): void {
        if (!this.socket) return;
        this.socket.onopen = () => this.onOpen();
        this.socket.onclose = () => this.onClose();
        this.socket.onerror = (event: Event) => this.onError(event);
        this.socket.onmessage = (event: MessageEvent) => this.onMessage(event);
    }

    private genericErrorToast(title: string, description: string): void {
        toast.error(title, {
            description
        });
    }

    private connectionToast(title: string, description: string, type: 'success' | 'info' | 'error'): void {
        if (this.connectionToastId) {
            toast.dismiss(this.connectionToastId);
        }
        this.connectionToastId = toast[type](title, {
            description,
            duration: 5000
        });
    }

    public send(data: U): void {
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(JSON.stringify(data));
        } else {
            this.genericErrorToast('Connection Error', 'You are not connected to the server.');
        }
    }

    private onOpen(): void {
        if (this.connectionToastId) {
            toast.dismiss(this.connectionToastId);
            this.connectionToastId = null;
        }
        if (!this.firstConnect) {
            this.connectionToast('Reconnected', 'Successfully reconnected to the WebSocket server.', 'success');
        }
        this.firstConnect = false;
        this.status = 'open';
        this.isReconnecting = false;
    }

    private onClose(): void {
        this.status = 'closed';
        this.isReconnecting = false;
        this.connectionToast('Connection Closed', 'Attempting to reconnect...', 'info');
        this.reconnectTimer = setTimeout(() => {
            this.reconnect();
        }, this.reconnectInterval);
    }

    private onError(event: Event): void {
        this.status = 'error';
        this.isReconnecting = false;
        this.connectionToast('Connection Error', event instanceof Error ? event.message : 'An error occurred with the WebSocket connection.', 'error');
        this.reconnectTimer = setTimeout(() => {
            this.reconnect();
        }, this.reconnectInterval);
    }

    private onMessage(event: MessageEvent): void {
        if (!this.messageHandler) {
            this.genericErrorToast('No Message Handler', 'No message handler is set to process incoming messages.');
            return;
        }
        try {
            let json = JSON.parse(event.data);
            this.messageHandler(json as T);
        } catch (error) {
            this.genericErrorToast('Failed to process message.', error instanceof Error ? error.message : 'Unknown error');
            console.trace(error);
        }
    }

}
