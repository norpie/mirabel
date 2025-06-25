import { goto } from "$app/navigation";
import type Result from "./models/result";
import { SocketHandler } from "./socket.svelte";

// TODO: Get the URL from the environment
const url = "http://localhost:8080/api";
// WebSocket URL derived from the HTTP URL
const wsUrl = url.replace(/^http/, 'ws');

// Add paths that don't require authentication
const publicPaths = [
  "v1/auth/register",
  "v1/auth/login",
  // Add other public endpoints
];

// Global state to prevent multiple refresh attempts
let isRefreshing = false;
let refreshPromise: Promise<Result<{ access_token: string }> | null> | null = null;

function formatEndpoint(path: string): string {
    return `${url}/${path}`;
}

function formatWebSocketEndpoint(path: string): string {
    return `${wsUrl}/${path}`;
}

function isPublicPath(endpoint: string): boolean {
  return publicPaths.some(path => endpoint.includes(path));
}

function connectWebSocket<T>(path: string, body?: any): SocketHandler<T> {
    let endpoint = formatWebSocketEndpoint(path);

    // Format query parameters
    let query = "";
    if (body) {
        query += "?";
        for (const key in body) {
            query += `${key}=${body[key]}&`;
        }
    }

    // Append access token as query parameter if available
    const accessToken = localStorage.getItem("accessToken");
    if (accessToken && !isPublicPath(path)) {
        if (query) {
            query += `access_token=${accessToken}`;
        } else {
            query += "?access_token=" + accessToken;
        }
    } else if (query) {
        // Remove the last '&' character if there's no token to add
        query = query.slice(0, -1);
    }

    return new SocketHandler(endpoint + query);
}

async function request<T>(method: string, endpoint: string, body: any | null = null, fetchFunction: typeof fetch = fetch): Promise<Result<T>> {
    // Don't add authorization header for public paths
    const headers: Record<string, string> = {
        "Content-Type": "application/json",
    };

    if (!isPublicPath(endpoint) && localStorage.getItem("accessToken")) {
        headers["Authorization"] = `Bearer ${localStorage.getItem("accessToken")}`;
    }

    const response = await fetchFunction(endpoint, {
        method,
        headers,
        body: body ? JSON.stringify(body) : undefined
    });

    // Only handle 401 for authenticated paths and if we're in a browser
    if (response.status === 401 && !isPublicPath(endpoint)) {
        const currentPath = window.location.pathname;

        // Don't try to refresh if we're already on login or register pages
        if (currentPath === "/login" || currentPath === "/register") {
            return {
                data: null,
                error: "Authentication required",
            };
        }

        // If already refreshing, wait for the existing refresh attempt
        if (isRefreshing && refreshPromise) {
            const tokenResult = await refreshPromise;

            if (tokenResult === null || tokenResult === undefined || tokenResult.error || !tokenResult.data) {
                return {
                    data: null,
                    error: "Token expired",
                };
            }

            // Retry with the new token
            localStorage.setItem("accessToken", tokenResult.data.access_token);
            return request(method, endpoint, body, fetchFunction);
        }

        // Start a new refresh attempt
        isRefreshing = true;
        refreshPromise = refresh(fetchFunction);

        try {
            const tokenResult = await refreshPromise;

            if (tokenResult === null || tokenResult === undefined || tokenResult.error || !tokenResult.data) {
                // Clear tokens and redirect to login
                localStorage.removeItem("accessToken");
                await goto("/login", {
                    invalidateAll: true,
                });
                return {
                    data: null,
                    error: "Token expired",
                };
            }

            localStorage.setItem("accessToken", tokenResult.data.access_token);
            return request(method, endpoint, body, fetchFunction);
        } finally {
            // Reset refresh state
            isRefreshing = false;
            refreshPromise = null;
        }
    }

    return await response.json();
}

async function get<T>(path: string, body?: any, fetchFunction: typeof fetch = fetch): Promise<Result<T>> {
    let query = "";
    if (body) {
        query += "?";
        for (const key in body) {
            query += `${key}=${body[key]}&`;
        }
        // Remove the last '&' character
        if (query.length > 0) {
            query = query.slice(0, -1);
        }
    }
    return request("GET", formatEndpoint(`${path}${query}`), null, fetchFunction);
}

async function post<T>(path: string, body: any, fetchFunction: typeof fetch = fetch): Promise<Result<T>> {
    return request("POST", formatEndpoint(path), body, fetchFunction);
}

async function put<T>(path: string, body: any, fetchFunction: typeof fetch = fetch): Promise<Result<T>> {
    return request("PUT", formatEndpoint(path), body, fetchFunction);
}

async function del<T>(path: string, fetchFunction: typeof fetch = fetch): Promise<Result<T>> {
    return request("DELETE", formatEndpoint(path), null, fetchFunction);
}

async function refresh(fetchFunction: typeof fetch = fetch): Promise<Result<{ access_token: string }> | null> {
    const response = await fetchFunction(formatEndpoint("v1/auth/refresh"), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
    });

    if (response.status === 401) {
        return null;
    }

    return await response.json();
}

export { isPublicPath, get, post, put, del, connectWebSocket };
