import type { User } from '$lib/models/user';
import type Result from '$lib/models/result';
import type { LoginFormData, RegisterFormData } from './schemas';
import { TokenStorage } from './tokens';

// Dynamically determine API URL based on current domain
function getApiUrl(): string {
    if (typeof window !== 'undefined') {
        const protocol = window.location.protocol;
        const hostname = window.location.hostname;
        const currentPort = parseInt(window.location.port) || (protocol === 'https:' ? 443 : 80);
        const apiPort = currentPort + 8080;
        return `${protocol}//${hostname}:${apiPort}/api`;
    }
    // Fallback for SSR
    return 'http://localhost:8080/api';
}

const API_BASE = getApiUrl();

export interface AuthResponse {
    access_token: string;
}

export interface AuthResult<T = AuthResponse> extends Result<T> {}

export class AuthAPI {
    private static formatEndpoint(path: string): string {
        return `${API_BASE}/${path}`;
    }

    static async login(credentials: LoginFormData): Promise<AuthResult> {
        try {
            const response = await fetch(this.formatEndpoint('v1/auth/login'), {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                credentials: 'include',
                body: JSON.stringify(credentials)
            });

            const result = await response.json();
            return result;
        } catch (error) {
            return {
                error: 'Network error occurred. Please try again.'
            };
        }
    }

    static async register(data: RegisterFormData): Promise<AuthResult> {
        try {
            const response = await fetch(this.formatEndpoint('v1/auth/register'), {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                credentials: 'include',
                body: JSON.stringify({
                    username: data.username,
                    email: data.email,
                    password: data.password
                })
            });

            const result = await response.json();
            return result;
        } catch (error) {
            return {
                error: 'Network error occurred. Please try again.'
            };
        }
    }

    static async refresh(): Promise<AuthResult> {
        try {
            const response = await fetch(this.formatEndpoint('v1/auth/refresh'), {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                credentials: 'include'
            });

            if (response.status === 401) {
                return {
                    error: 'Session expired'
                };
            }

            const result = await response.json();
            return result;
        } catch (error) {
            return {
                error: 'Network error occurred. Please try again.'
            };
        }
    }

    static async logout(): Promise<void> {
        try {
            await fetch(this.formatEndpoint('v1/auth/logout'), {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json'
                },
                credentials: 'include'
            });
        } catch (error) {
            console.error('Logout request failed:', error);
        }
    }

    static async getCurrentUser(fetchFunction: typeof fetch = fetch): Promise<Result<User>> {
        try {
            const headers: Record<string, string> = {
                'Content-Type': 'application/json'
            };

            // Add authorization header if we have a token
            const accessToken = TokenStorage.getAccessToken();
            if (accessToken) {
                headers['Authorization'] = `Bearer ${accessToken}`;
            }

            const response = await fetchFunction(this.formatEndpoint('v1/me'), {
                method: 'GET',
                headers,
                credentials: 'include'
            });

            const result = await response.json();
            return result;
        } catch (error) {
            return {
                error: 'Failed to fetch user data'
            };
        }
    }
}
