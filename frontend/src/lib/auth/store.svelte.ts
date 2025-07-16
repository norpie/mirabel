import { goto } from '$app/navigation';
import { toast } from 'svelte-sonner';
import type { User } from '$lib/models/user';
import type { LoginFormData, RegisterFormData } from './schemas';
import { AuthAPI } from './api';
import { TokenStorage } from './tokens';

class AuthStore {
	private _user = $state<User | null>(null);
	private _isLoading = $state(false);
	private _isInitialized = $state(false);

	get user(): User | null {
		return this._user;
	}

	get isAuthenticated(): boolean {
		return this._user !== null && TokenStorage.hasAccessToken();
	}

	get isLoading(): boolean {
		return this._isLoading;
	}

	get isInitialized(): boolean {
		return this._isInitialized;
	}

	async initialize(): Promise<void> {
		if (this._isInitialized) return;

		this._isLoading = true;

		try {
			// Check if we have a token
			if (TokenStorage.hasAccessToken()) {
				const userResult = await AuthAPI.getCurrentUser();
				if (userResult.data) {
					this._user = userResult.data;
				} else {
					// Token is invalid, try to refresh
					await this.refreshToken();
				}
			} else {
				// No token, try to refresh from cookie
				await this.refreshToken();
			}
		} catch (error) {
			console.error('Auth initialization failed:', error);
		} finally {
			this._isLoading = false;
			this._isInitialized = true;
		}
	}

	async login(credentials: LoginFormData): Promise<{ success: boolean; error?: string }> {
		this._isLoading = true;

		try {
			const result = await AuthAPI.login(credentials);

			if (result.error || !result.data) {
				return {
					success: false,
					error: result.error || 'Login failed'
				};
			}

			TokenStorage.setAccessToken(result.data.access_token);

			// Fetch user data
			const userResult = await AuthAPI.getCurrentUser();
			if (userResult.data) {
				this._user = userResult.data;
				toast.success('Logged in successfully');
				await goto('/', { invalidateAll: true });
				return { success: true };
			} else {
				TokenStorage.clearAll();
				return {
					success: false,
					error: 'Failed to fetch user data'
				};
			}
		} catch (error) {
			return {
				success: false,
				error: 'An unexpected error occurred'
			};
		} finally {
			this._isLoading = false;
		}
	}

	async register(data: RegisterFormData): Promise<{ success: boolean; error?: string }> {
		this._isLoading = true;

		try {
			const result = await AuthAPI.register(data);

			if (result.error || !result.data) {
				return {
					success: false,
					error: result.error || 'Registration failed'
				};
			}

			TokenStorage.setAccessToken(result.data.access_token);

			// Fetch user data
			const userResult = await AuthAPI.getCurrentUser();
			if (userResult.data) {
				this._user = userResult.data;
				toast.success('Account created successfully');
				await goto('/', { invalidateAll: true });
				return { success: true };
			} else {
				TokenStorage.clearAll();
				return {
					success: false,
					error: 'Failed to fetch user data'
				};
			}
		} catch (error) {
			return {
				success: false,
				error: 'An unexpected error occurred'
			};
		} finally {
			this._isLoading = false;
		}
	}

	async logout(): Promise<void> {
		this._isLoading = true;

		try {
			await AuthAPI.logout();
		} catch (error) {
			console.error('Logout API call failed:', error);
		} finally {
			TokenStorage.clearAll();
			this._user = null;
			this._isLoading = false;
			toast.success('Logged out successfully');
			await goto('/login', { invalidateAll: true });
		}
	}

	async refreshToken(): Promise<boolean> {
		try {
			const result = await AuthAPI.refresh();

			if (result.error || !result.data) {
				TokenStorage.clearAll();
				this._user = null;
				return false;
			}

			TokenStorage.setAccessToken(result.data.access_token);

			// Fetch updated user data
			const userResult = await AuthAPI.getCurrentUser();
			if (userResult.data) {
				this._user = userResult.data;
				return true;
			} else {
				TokenStorage.clearAll();
				this._user = null;
				return false;
			}
		} catch (error) {
			console.error('Token refresh failed:', error);
			TokenStorage.clearAll();
			this._user = null;
			return false;
		}
	}

	setUser(user: User): void {
		this._user = user;
	}

	clearAuth(): void {
		TokenStorage.clearAll();
		this._user = null;
	}
}

export const authStore = new AuthStore();