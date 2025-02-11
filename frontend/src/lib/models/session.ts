export interface ShallowSession {
    id: string;
    title: string;
}

export interface Session {
    id: string;
    title: string;
    participants: Participant[];
    chat: Chat;
}

export interface Participant {
    id: string;
    name: string;
    user: boolean;
    avatar?: string;
}

export interface Chat {
    messages: Message[];
}

export interface Message {
    timestamp: string;
    participant: string;
    message: string;
}
