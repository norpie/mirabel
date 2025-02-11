export interface ShallowSession {
    id: string;
    title: string;
}

export interface Session {
    id: string;
    title: string;
    chat: Chat;
}

export interface Chat {
    messages: Message[];
}

export interface Message {
    timestamp: string;
    author: string;
    message: string;
}
