export type TaskType = 'planned' | 'detour-insert' | 'detour-around';

export type TaskStatus = 'pending' | 'in-progress' | 'completed' | 'cancelled' | 'paused';

export interface PlanTask {
    id: string;
    title: string;
    type: TaskType;
    status: TaskStatus;
    children: PlanTask[];
    createdAt: Date;
    startedAt?: Date;
    completedAt?: Date;
    pausedAt?: Date;
    cancelledAt?: Date;
    detourReason?: string;
    originalTaskId?: string;
}

export interface Plan {
    id: string;
    title: string;
    tasks: PlanTask[];
    createdAt: Date;
    updatedAt?: Date;
    workspaceId: string;
    sessionId: string;
    currentTaskId?: string;
}

export function createTask(title: string, type: TaskType = 'planned'): PlanTask {
    return {
        id: crypto.randomUUID(),
        title,
        type,
        status: 'pending',
        children: [],
        createdAt: new Date()
    };
}

export function placeholderPlan(): Plan {
    return {
        id: 'placeholder',
        title: 'Authentication & Session Management Implementation',
        tasks: [
            {
                id: '1',
                title: 'Setup development environment and dependencies',
                type: 'planned',
                status: 'completed',
                children: [
                    {
                        id: '1-1',
                        title: 'Initialize Rust backend with Axum framework',
                        type: 'planned',
                        status: 'completed',
                        children: [
                            {
                                id: '1-1-1',
                                title: 'Create Cargo.toml with axum, tokio, and diesel',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '1-1-2',
                                title: 'Setup main.rs with basic HTTP server',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    },
                    {
                        id: '1-2',
                        title: 'Configure PostgreSQL database connection',
                        type: 'planned',
                        status: 'completed',
                        children: [
                            {
                                id: '1-2-1',
                                title: 'Setup diesel CLI and database URL',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '1-2-2',
                                title: 'Create initial migration for users table',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    }
                ],
                createdAt: new Date()
            },
            {
                id: '2',
                title: 'Implement GitHub OAuth authentication flow',
                type: 'planned',
                status: 'completed',
                children: [
                    {
                        id: '2-1',
                        title: 'Setup OAuth application and configuration',
                        type: 'planned',
                        status: 'completed',
                        children: [
                            {
                                id: '2-1-1',
                                title: 'Register GitHub OAuth app with callback URL',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '2-1-2',
                                title: 'Add client ID and secret to environment config',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    },
                    {
                        id: '2-2',
                        title: 'Build OAuth redirect and callback handlers',
                        type: 'planned',
                        status: 'completed',
                        children: [
                            {
                                id: '2-2-1',
                                title: 'Implement github_auth_redirect() with state parameter',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '2-2-2',
                                title: 'Create callback handler with token exchange',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '2-2-3',
                                title: 'Add user profile fetching from GitHub API',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    }
                ],
                createdAt: new Date()
            },
            {
                id: '3',
                title: 'Build user management and JWT session system',
                type: 'planned',
                status: 'completed',
                children: [
                    {
                        id: '3-1',
                        title: 'Create user database models and operations',
                        type: 'planned',
                        status: 'completed',
                        children: [
                            {
                                id: '3-1-1',
                                title: 'Define User struct with GitHub integration fields',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '3-1-2',
                                title: 'Implement create_or_update_user() function',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    },
                    {
                        id: '3-2',
                        title: 'Implement JWT token management',
                        type: 'planned',
                        status: 'completed',
                        children: [
                            {
                                id: '3-2-1',
                                title: 'Setup jsonwebtoken dependency and secret config',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '3-2-2',
                                title: 'Create generate_jwt_token() function',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '3-2-3',
                                title: 'Build JWT authentication middleware',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    },
                    {
                        id: '3-3',
                        title: 'Fix token validation edge cases',
                        type: 'detour-insert',
                        status: 'completed',
                        detourReason:
                            'Discovered expired tokens were not handled properly during testing',
                        children: [
                            {
                                id: '3-3-1',
                                title: 'Add proper error handling for expired tokens',
                                type: 'detour-insert',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '3-3-2',
                                title: 'Implement token refresh mechanism',
                                type: 'detour-insert',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    }
                ],
                createdAt: new Date()
            },
            {
                id: '4',
                title: 'Fix critical database connection pooling issue',
                type: 'detour-insert',
                status: 'completed',
                detourReason:
                    'Discovered connection pool exhaustion causing timeouts during OAuth testing',
                children: [
                    {
                        id: '4-1',
                        title: 'Investigate connection pool configuration',
                        type: 'detour-insert',
                        status: 'completed',
                        children: [
                            {
                                id: '4-1-1',
                                title: 'Analyze current pool settings in database.rs',
                                type: 'detour-insert',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '4-1-2',
                                title: 'Review connection timeout logs',
                                type: 'detour-insert',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    },
                    {
                        id: '4-2',
                        title: 'Implement proper connection pool management',
                        type: 'detour-insert',
                        status: 'completed',
                        children: [
                            {
                                id: '4-2-1',
                                title: 'Increase max_connections to 20',
                                type: 'detour-insert',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '4-2-2',
                                title: 'Add connection timeout handling',
                                type: 'detour-insert',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    }
                ],
                createdAt: new Date()
            },
            {
                id: '5',
                title: 'Create workspace management system',
                type: 'planned',
                status: 'in-progress',
                children: [
                    {
                        id: '5-1',
                        title: 'Design workspace database schema',
                        type: 'planned',
                        status: 'completed',
                        children: [
                            {
                                id: '5-1-1',
                                title: 'Create workspaces table migration',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '5-1-2',
                                title: 'Add workspace_members association table',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    },
                    {
                        id: '5-2',
                        title: 'Implement workspace CRUD operations',
                        type: 'planned',
                        status: 'in-progress',
                        children: [
                            {
                                id: '5-2-1',
                                title: 'Build create_workspace() function',
                                type: 'planned',
                                status: 'completed',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '5-2-2',
                                title: 'Implement full workspace API with complex permissions',
                                type: 'planned',
                                status: 'cancelled',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '5-2-3',
                                title: 'Build simplified workspace API without permissions',
                                type: 'detour-around',
                                status: 'in-progress',
                                detourReason:
                                    'Original complex permissions approach was too time-consuming, switching to MVP approach',
                                originalTaskId: '5-2-2',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    }
                ],
                createdAt: new Date()
            },
            {
                id: '7',
                title: 'Debug workspace creation memory leak',
                type: 'detour-insert',
                status: 'pending',
                detourReason:
                    'Memory usage spiking during workspace creation testing - need to investigate before continuing',
                children: [
                    {
                        id: '7-1',
                        title: 'Profile memory usage during workspace operations',
                        type: 'detour-insert',
                        status: 'pending',
                        children: [
                            {
                                id: '7-1-1',
                                title: 'Run valgrind on workspace creation endpoint',
                                type: 'detour-insert',
                                status: 'pending',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '7-1-2',
                                title: 'Analyze heap allocations in diesel ORM calls',
                                type: 'detour-insert',
                                status: 'pending',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    },
                    {
                        id: '7-2',
                        title: 'Fix identified memory management issues',
                        type: 'detour-insert',
                        status: 'pending',
                        children: [
                            {
                                id: '7-2-1',
                                title: 'Replace Vec clones with references in workspace_service',
                                type: 'detour-insert',
                                status: 'pending',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '7-2-2',
                                title: 'Add explicit drop for large query results',
                                type: 'detour-insert',
                                status: 'pending',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    }
                ],
                createdAt: new Date()
            },
            {
                id: '8',
                title: 'Build frontend authentication and user interface',
                type: 'planned',
                status: 'pending',
                children: [
                    {
                        id: '8-1',
                        title: 'Setup Svelte frontend with authentication state',
                        type: 'planned',
                        status: 'pending',
                        children: [
                            {
                                id: '8-1-1',
                                title: 'Create auth store using Svelte 5 runes',
                                type: 'planned',
                                status: 'pending',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '8-1-2',
                                title: 'Implement login/logout UI components',
                                type: 'planned',
                                status: 'pending',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    },
                    {
                        id: '8-2',
                        title: 'Create workspace dashboard interface',
                        type: 'planned',
                        status: 'pending',
                        children: [
                            {
                                id: '8-2-1',
                                title: 'Build workspace list and card components',
                                type: 'planned',
                                status: 'pending',
                                children: [],
                                createdAt: new Date()
                            },
                            {
                                id: '8-2-2',
                                title: 'Add workspace creation modal with validation',
                                type: 'planned',
                                status: 'pending',
                                children: [],
                                createdAt: new Date()
                            }
                        ],
                        createdAt: new Date()
                    }
                ],
                createdAt: new Date()
            }
        ],
        createdAt: new Date(),
        workspaceId: 'workspace-1',
        sessionId: 'session-1',
        currentTaskId: '5-2-3'
    };
}
