export interface User {
    id: string;
    email: string;
    username: string;
    avatar: string;
}

export function emptyUser(): User {
    return {
        id: '',
        email: '',
        username: '',
        avatar: ''
    };
}