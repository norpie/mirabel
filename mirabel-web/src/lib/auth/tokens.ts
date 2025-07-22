const ACCESS_TOKEN_KEY = 'access_token';

export class TokenStorage {
    static setAccessToken(token: string): void {
        if (typeof window !== 'undefined') {
            sessionStorage.setItem(ACCESS_TOKEN_KEY, token);
        }
    }

    static getAccessToken(): string | null {
        if (typeof window !== 'undefined') {
            return sessionStorage.getItem(ACCESS_TOKEN_KEY);
        }
        return null;
    }

    static removeAccessToken(): void {
        if (typeof window !== 'undefined') {
            sessionStorage.removeItem(ACCESS_TOKEN_KEY);
        }
    }

    static hasAccessToken(): boolean {
        return this.getAccessToken() !== null;
    }

    static clearAll(): void {
        this.removeAccessToken();
    }
}
