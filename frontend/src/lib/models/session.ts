export interface ShallowSession {
    id: string;
    title: string;
}

export interface Session {
    id: string;
    title: string;
    participants: Participant[];
    chat: Chat;
    plan: Plan | undefined;
    terminal: string[] | undefined;
}

export interface Plan {
    goal: string;
    spec: string;
    children: PlanItem[];
}

export interface PlanItem {
    id: string;
    name: string;
    description: string;
    status: 'done' | 'paused' | 'in-progress' | 'todo';
    children: PlanItem[];
}

export interface Participant {
    id: string;
    name: string;
    avatar?: string;
}

export interface Chat {
    messages: Message[];
}

export interface Message {
    timestamp: string;
    authorId: string;
    message: string;
}
