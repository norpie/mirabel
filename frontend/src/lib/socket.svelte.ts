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
    private reconnectInterval: number = 3000;
    private reconnectTimer: ReturnType<typeof setTimeout> | null = null;

    // Toast management
    private currentToastId: string | number | null = null;
    private retryCountdownInterval: ReturnType<typeof setInterval> | null = null;

    constructor(endpoint: string) {
        this.endpoint = endpoint;
    }

    public connect(): void {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
            return;
        }
        
        try {
            this.status = 'connecting';
            this.socket = new WebSocket(this.endpoint);
            this.setHandlers();
        } catch (error) {
            this.status = 'error';
            if (this.firstConnect) {
                this.showConnectionWarning();
            }
            console.log(error);
        }
        // Don't reset isReconnecting here - let onOpen handle it
    }

    public disconnect(): void {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
        this.dismissCurrentToast();
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
        this.dismissCurrentToast();
        this.connect();
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

    public send(data: U): void {
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(JSON.stringify(data));
        }
    }

    private onOpen(): void {
        const wasReconnecting = this.isReconnecting;
        this.firstConnect = false;
        this.status = 'open';
        this.isReconnecting = false;
        
        this.dismissCurrentToast();
        
        if (wasReconnecting) {
            this.currentToastId = toast.success("Reconnected to server", {
                description: "Connection restored successfully",
                duration: 3000
            });
        }
    }

    private onClose(): void {
        this.status = 'closed';
        this.isReconnecting = false;
        
        // Show connection lost error if this wasn't the first connection
        if (!this.firstConnect) {
            this.showConnectionLostError();
        }
        
        this.reconnectTimer = setTimeout(() => {
            this.reconnect();
        }, this.reconnectInterval);
    }

    private onError(event: Event): void {
        this.status = 'error';
        this.isReconnecting = false;
        
        // Show appropriate error toast
        if (this.firstConnect) {
            this.showConnectionWarning();
        } else {
            this.showConnectionLostError();
        }
        
        this.reconnectTimer = setTimeout(() => {
            this.reconnect();
        }, this.reconnectInterval);
    }

    private onMessage(event: MessageEvent): void {
        if (!this.messageHandler) {
            return;
        }
        try {
            let json = JSON.parse(event.data);
            this.messageHandler(json as T);
        } catch (error) {
            console.trace(error);
        }
    }

    private dismissCurrentToast(): void {
        if (this.retryCountdownInterval) {
            clearInterval(this.retryCountdownInterval);
            this.retryCountdownInterval = null;
        }
        toast.dismiss();
        this.currentToastId = null;
    }

    private showConnectionWarning(): void {
        this.dismissCurrentToast();
        this.currentToastId = toast.warning("Failed to connect to server", {
            description: "Please check your internet connection",
            duration: 5000
        });
    }

    private showConnectionLostError(): void {
        this.dismissCurrentToast();
        
        let countdown = Math.ceil(this.reconnectInterval / 1000);
        
        this.currentToastId = toast.error("Connection lost", {
            description: `Retrying in ${countdown} second${countdown === 1 ? '' : 's'}`,
            duration: Infinity,
            action: {
                label: "Retry now",
                onClick: () => {
                    this.dismissCurrentToast();
                    this.manualReconnect();
                }
            }
        });
        
        this.retryCountdownInterval = setInterval(() => {
            countdown--;
            if (countdown <= 0) {
                this.dismissCurrentToast();
                return;
            }
            
            toast.error("Connection lost", {
                id: this.currentToastId,
                description: `Retrying in ${countdown} second${countdown === 1 ? '' : 's'}`,
                duration: Infinity,
                action: {
                    label: "Retry now",
                    onClick: () => {
                        this.dismissCurrentToast();
                        this.manualReconnect();
                    }
                }
            });
        }, 1000);
    }

}
