import { emptySession, type Session, type TimelineEntry, type UserInteraction } from "./models/session";
import type { SocketHandler } from "./socket.svelte";
import { emptyUser, type User } from "./models/user";

export class SessionState {
    user: User = $state(emptyUser());
    timeline: TimelineEntry[] = $state([]);
    timelineKnownTotal: number = $state(0);
    session: Session = $state(emptySession());
    socket: SocketHandler<TimelineEntry, UserInteraction> | undefined = $state();

    newMessageCallback: () => void = () => {};

    lastAcknowledgementTime: Date | undefined = $state();
    lastAcknowledgementType: 'sent' | 'delivered' | 'seen' | undefined = $state();

    agentStatus: 'thinking' | 'typing' | 'paused' | 'error' | undefined = $state();
    agentStatusTime: Date | undefined = $state();

    constructor(user: User, session: Session, socket: SocketHandler<TimelineEntry, UserInteraction>) {
        this.user = user;
        this.session = session;
        this.timeline = session.timeline.data;
        this.timelineKnownTotal = session.timeline.pageInfo.total;
        this.socket = socket;
        this.socket.setMessageHandler(this.onEvent.bind(this));
    }

    public setNewMessageCallback(callback: () => void): void {
        this.newMessageCallback = callback;
    }

    public removeNewMessageCallback(): void {
        this.newMessageCallback = () => {};
    }

    private onEvent(event: TimelineEntry): void {
        this.timeline.push(event);
        this.timelineKnownTotal += 1;
        switch (event.content.type) {
            case 'acknowledgment':
                this.onAcknowledge.bind(this)(event);
                break;
            case 'agentStatus':
                this.onAgentStatus.bind(this)(event);
                break;
            case 'message':
                this.onMessageContent.bind(this)(event);
                break;
            default:
                console.warn(`Unhandled session event type: ${event.content.type}`);
        }
    }

    private onAcknowledge(event: TimelineEntry): void {
        this.lastAcknowledgementTime = new Date(event.createdAt);
        this.lastAcknowledgementType = event.content.ackType;
    }

    private onAgentStatus(event: TimelineEntry): void {
        this.agentStatusTime = new Date(event.createdAt);
        this.agentStatus = event.content.status;
    }

    private onMessageContent(event: TimelineEntry): void {
        this.lastAcknowledgementType = undefined;
        this.lastAcknowledgementTime = undefined;
        this.agentStatus = undefined;
        this.agentStatusTime = undefined;
    }
}
