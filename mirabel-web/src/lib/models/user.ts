import type { FrontendUser } from '../generated';

export type User = FrontendUser;

export function emptyUser(): User {
    return {
        id: '',
        email: '',
        username: '',
        createdAt: ''
    };
}
