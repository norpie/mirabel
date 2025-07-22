import { toast } from 'svelte-sonner';
import { authStore } from './auth/store.svelte';
import { TokenStorage } from './auth/tokens';
import { goto } from '$app/navigation';

export class SocketHandler<T, U> {
    // Internal state
    private baseEndpoint: string;
    private queryParams: Record<string, string>;
    private firstConnect: boolean = true;
    private socket: WebSocket | null = null;
    status: 'connecting' | 'open' | 'closing' | 'closed' | 'error' = $state('closed');

    // Auth handling
    private requiresAuth: boolean = true;
    private lastAuthToken: string | null = null;

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

    constructor(
        baseEndpoint: string,
        queryParams: Record<string, string> = {},
        requiresAuth: boolean = true
    ) {
        // Parse the baseEndpoint to separate URL and query params
        const [url, existingQuery] = baseEndpoint.split('?');
        this.baseEndpoint = url;
        this.queryParams = { ...queryParams };
        this.requiresAuth = requiresAuth;

        // Parse existing query params if any
        if (existingQuery) {
            const urlParams = new URLSearchParams(existingQuery);
            for (const [key, value] of urlParams) {
                this.queryParams[key] = value;
            }
        }
    }

    private buildEndpointWithAuth(): string {
        let params = { ...this.queryParams };

        if (this.requiresAuth) {
            const accessToken = TokenStorage.getAccessToken();
            if (accessToken) {
                params.access_token = accessToken;
                this.lastAuthToken = accessToken;
            } else if (authStore.isAuthenticated) {
                // Auth store says we're authenticated but no token available
                // This is an inconsistent state - force logout
                console.warn('Auth store claims authenticated but no access token available');
                authStore.logout();
                return '';
            } else {
                // Not authenticated and auth is required
                console.warn('WebSocket requires authentication but user is not authenticated');
                return '';
            }
        }

        const queryString = new URLSearchParams(params).toString();
        return `${this.baseEndpoint}${queryString ? '?' + queryString : ''}`;
    }

    private shouldAttemptReconnect(): boolean {
        if (!this.autoReconnect) return false;

        // If auth is required, only reconnect if user is still authenticated
        if (this.requiresAuth && !authStore.isAuthenticated) {
            console.log('Skipping reconnect: user is no longer authenticated');
            return false;
        }

        // Check if token has changed (might have been refreshed)
        if (this.requiresAuth) {
            const currentToken = TokenStorage.getAccessToken();
            if (currentToken !== this.lastAuthToken) {
                console.log('Access token changed, will use new token for reconnect');
                return true;
            }
        }

        return true;
    }

    public connect(): void {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
            return;
        }

        // Check auth requirements before connecting
        if (this.requiresAuth && !authStore.isAuthenticated) {
            this.status = 'error';
            console.warn(
                'Cannot connect WebSocket: authentication required but user not authenticated'
            );
            return;
        }

        const endpoint = this.buildEndpointWithAuth();
        if (!endpoint) {
            this.status = 'error';
            if (this.firstConnect) {
                this.showAuthError();
            }
            return;
        }

        try {
            this.status = 'connecting';
            this.socket = new WebSocket(endpoint);
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
        if (!this.shouldAttemptReconnect() || this.isReconnecting) return;
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
        this.socket.onclose = (event: CloseEvent) => this.onClose(event);
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
            this.currentToastId = toast.success('Reconnected to server', {
                description: 'Connection restored successfully',
                duration: 3000
            });
        }
    }

    private onClose(event: CloseEvent): void {
        this.status = 'closed';
        this.isReconnecting = false;

        // Check for auth-related close codes
        if (event.code === 1008 || event.code === 4001 || event.code === 4003) {
            // Authentication-related close codes
            this.handleAuthError();
            return;
        }

        // Show connection lost error if this wasn't the first connection
        if (!this.firstConnect) {
            this.showConnectionLostError();
        }

        if (this.shouldAttemptReconnect()) {
            this.reconnectTimer = setTimeout(() => {
                this.reconnect();
            }, this.reconnectInterval);
        }
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

        if (this.shouldAttemptReconnect()) {
            this.reconnectTimer = setTimeout(() => {
                this.reconnect();
            }, this.reconnectInterval);
        }
    }

    private onMessage(event: MessageEvent): void {
        if (!this.messageHandler) {
            return;
        }
        try {
            let json = JSON.parse(event.data);

            // Check for auth error messages from server
            if (json.type === 'auth_error' || json.error === 'unauthorized') {
                this.handleAuthError();
                return;
            }

            this.messageHandler(json as T);
        } catch (error) {
            console.trace(error);
        }
    }

    private handleAuthError(): void {
        this.dismissCurrentToast();
        this.autoReconnect = false; // Stop reconnection attempts

        if (this.requiresAuth) {
            console.warn('WebSocket authentication failed - logging out user');

            // Show auth error toast
            this.currentToastId = toast.error('Authentication expired', {
                description: 'Your session has expired. Please log in again.',
                duration: 5000,
                action: {
                    label: 'Login',
                    onClick: () => {
                        goto('/login');
                    }
                }
            });

            // Force logout through auth store
            authStore.logout();
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
        this.currentToastId = toast.warning('Failed to connect to server', {
            description: 'Please check your internet connection',
            duration: 5000
        });
    }

    private showAuthError(): void {
        this.dismissCurrentToast();
        this.currentToastId = toast.error('Authentication required', {
            description: 'Please log in to access this feature',
            duration: 5000,
            action: {
                label: 'Login',
                onClick: () => {
                    goto('/login');
                }
            }
        });
    }

    private showConnectionLostError(): void {
        this.dismissCurrentToast();

        if (!this.shouldAttemptReconnect()) {
            // Don't show retry countdown if we won't actually retry
            this.currentToastId = toast.error('Connection lost', {
                description: 'Unable to reconnect - please refresh the page',
                duration: Infinity,
                action: {
                    label: 'Refresh',
                    onClick: () => {
                        window.location.reload();
                    }
                }
            });
            return;
        }

        let countdown = Math.ceil(this.reconnectInterval / 1000);

        this.currentToastId = toast.error('Connection lost', {
            description: `Retrying in ${countdown} second${countdown === 1 ? '' : 's'}`,
            duration: Infinity,
            action: {
                label: 'Retry now',
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

            toast.error('Connection lost', {
                id: this.currentToastId,
                description: `Retrying in ${countdown} second${countdown === 1 ? '' : 's'}`,
                duration: Infinity,
                action: {
                    label: 'Retry now',
                    onClick: () => {
                        this.dismissCurrentToast();
                        this.manualReconnect();
                    }
                }
            });
        }, 1000);
    }
}
