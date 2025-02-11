export interface ShallowSession {
    id: string;
    title: string;
}

export interface Session {
    id: string;
    title: string;
    participants: Participant[];
    chat: Chat;
    plan: Plan;
}

export interface Plan {
    goal: string;
    spec: string;
    children: PlanItem[];
}

export interface PlanItem {
    name: string;
    description: string;
    status: 'done' | 'in-progress' | 'todo';
    children: PlanItem[];
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
