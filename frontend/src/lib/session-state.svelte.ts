import { getContext, setContext } from "svelte";
import { emptySession, type Session } from "./models/session";
import type { SocketHandler } from "./socket.svelte";
import type { SessionAcknowledgmentEvent, SessionEvent, SessionMessageEvent } from "./models/event";
import { emptyUser, type User } from "./models/user";

export class SessionState {
    user: User = $state(emptyUser());
    session: Session = $state(emptySession());
    socket: SocketHandler<SessionEvent> | undefined = $state();

    newMessageCallback: () => void = () => {};

    lastAcknowledgementTime: Date | undefined = $state();
    lastAcknowledgementType: 'sent' | 'delivered' | 'seen' | 'thinking' | 'typing' | 'paused' | 'error' | undefined = $state();

    constructor(user: User, session: Session, socket: SocketHandler<SessionEvent>) {
        this.user = user;
        this.session = session;
        this.socket = socket;
        this.socket.setMessageHandler(this.onEvent.bind(this));
    }

    public setNewMessageCallback(callback: () => void): void {
        this.newMessageCallback = callback;
    }

    public removeNewMessageCallback(): void {
        this.newMessageCallback = () => {};
    }

    private onEvent(event: SessionEvent): void {
        switch (event.content.type) {
            case 'AcknowledgmentContent':
                this.onAcknowledge.bind(this)(event as SessionAcknowledgmentEvent);
                break;
            case 'MessageContent':
                this.onMessageContent.bind(this)(event as SessionMessageEvent);
                break;
            default:
                console.warn(`Unhandled session event type: ${event.content.type}`);
        }
    }

    private onAcknowledge(event: SessionAcknowledgmentEvent): void {
        this.lastAcknowledgementTime = new Date(event.timestamp);
        this.lastAcknowledgementType = event.content.ackType;
    }

    private onMessageContent(event: SessionMessageEvent): void {
        if (!this.session?.chat) {
            console.warn("Received message event for a session/chat that is not set.");
            return;
        }
        this.session.chat.messages = [...this.session.chat.messages, {
            timestamp: event.timestamp,
            authorId: event.content.authorId,
            message: event.content.message
        }];
        this.lastAcknowledgementTime = new Date(event.timestamp);
        this.lastAcknowledgementType = 'sent';
    }
}

const SESSION_KEY = Symbol("SESSION");

export function setSessionState(user: User, session: Session, socket: SocketHandler<SessionEvent>) {
    return setContext(SESSION_KEY, new SessionState(user, session, socket));
}

export function getSessionState() {
    return getContext<ReturnType<typeof setSessionState>>(SESSION_KEY);
}
