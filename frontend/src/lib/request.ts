import { goto } from "$app/navigation";
import type Result from "../models/result";
import { browser } from "$app/environment";

// TODO: Get the URL from the environment
const url = "http://localhost:8080/api";

// Add paths that don't require authentication
const publicPaths = [
  "v1/auth/register",
  "v1/auth/login",
  // Add other public endpoints
];

function formatEndpoint(path: string): string {
    return `${url}/${path}`;
}

function isPublicPath(endpoint: string): boolean {
  return publicPaths.some(path => endpoint.includes(path));
}

async function request<T>(method: string, endpoint: string, body?: any): Promise<Result<T>> {
    // Don't add authorization header for public paths
    const headers: Record<string, string> = {
        "Content-Type": "application/json",
    };
    
    if (!isPublicPath(endpoint) && browser && localStorage.getItem("accessToken")) {
        headers["Authorization"] = `Bearer ${localStorage.getItem("accessToken")}`;
    }
    
    const response = await fetch(endpoint, {
        method,
        headers,
        body: body ? JSON.stringify(body) : undefined
    });

    // Only handle 401 for authenticated paths and if we're in a browser
    if (response.status === 401 && !isPublicPath(endpoint) && browser) {
        const currentPath = window.location.pathname;
        
        // Don't try to refresh if we're already on login or register pages
        if (currentPath === "/login" || currentPath === "/register") {
            return {
                data: null,
                error: "Authentication required",
            };
        }
        
        const tokenResult = await refresh();

        if (tokenResult === null || tokenResult === undefined || tokenResult.error || !tokenResult.data) {
            goto("/login");
            return {
                data: null,
                error: "Token expired",
            };
        }

        localStorage.setItem("accessToken", tokenResult.data.access_token);
        return request(method, endpoint, body);
    }

    return await response.json();
}

async function get<T>(path: string, body?: any): Promise<Result<T>> {
    let query = "";
    if (body) {
        for (const key in body) {
            query += `${key}=${body[key]}&`;
        }
        // Remove the last '&' character
        if (query.length > 0) {
            query = query.slice(0, -1);
        }
    }
    return request("GET", formatEndpoint(`${path}?${query}`));
}

async function post<T>(path: string, body: any): Promise<Result<T>> {
    return request("POST", formatEndpoint(path), body);
}

async function put<T>(path: string, body: any): Promise<Result<T>> {
    return request("PUT", formatEndpoint(path), body);
}

async function del<T>(path: string): Promise<Result<T>> {
    return request("DELETE", formatEndpoint(path));
}

async function refresh(): Promise<Result<{ access_token: string }> | null> {
    const response = await fetch(formatEndpoint("v1/auth/refresh"), {
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

export { get, post, put, del };
