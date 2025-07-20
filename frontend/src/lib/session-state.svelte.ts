import {
    emptySession,
    type Session,
    type TimelineEntry,
    type UserInteraction
} from './models/session';
import type { SocketHandler } from './socket.svelte';
import { emptyUser, type User } from './models/user';
import { getSessionTimelineCursor } from './api/session';

export class SessionState {
    user: User = $state(emptyUser());
    timeline: TimelineEntry[] = $state([]);
    timelineKnownTotal: number = $state(0);
    session: Session = $state(emptySession());
    socket: SocketHandler<TimelineEntry, UserInteraction> | undefined = $state();
    
    // Infinite scroll state
    isLoadingOlder: boolean = $state(false);
    hasMoreOlder: boolean = $state(true);
    oldestCursor: string | undefined = $state();

    newMessageCallback: () => void = () => {};

    lastAcknowledgementTime: Date | undefined = $state();
    lastAcknowledgementType: 'sent' | 'delivered' | 'seen' | undefined = $state();

    agentStatus: 'thinking' | 'typing' | 'paused' | 'error' | undefined = $state();
    agentStatusTime: Date | undefined = $state();

    constructor(
        user: User,
        session: Session,
        socket: SocketHandler<TimelineEntry, UserInteraction>
    ) {
        this.user = user;
        this.session = session;
        this.timeline = session.timeline.data;
        this.timelineKnownTotal = session.timeline.pageInfo.total;
        this.socket = socket;
        this.socket.setMessageHandler(this.onEvent.bind(this));
        
        // Set initial cursor to the oldest message (first in chronological order)
        if (this.timeline.length > 0) {
            this.oldestCursor = this.timeline[0].createdAt;
        }
    }

    public async loadOlderMessages(): Promise<void> {
        if (this.isLoadingOlder || !this.hasMoreOlder || !this.oldestCursor) {
            return;
        }

        this.isLoadingOlder = true;
        
        try {
            // Extract workspace ID from session context or URL
            const workspaceId = window.location.pathname.split('/')[2]; // Assuming URL structure /workspace/[id]/...
            
            const response = await getSessionTimelineCursor(
                workspaceId,
                this.session.id,
                {
                    before: this.oldestCursor,
                    limit: 20
                }
            );
            
            // Extract the actual cursor response from the API wrapper
            const cursorResponse = response.data;
            
            if (cursorResponse.data.length > 0) {
                // Prepend older messages to the beginning
                this.timeline = [...cursorResponse.data, ...this.timeline];
                // Update cursor to the oldest message we just loaded
                this.oldestCursor = cursorResponse.data[0].createdAt;
                this.hasMoreOlder = cursorResponse.hasMore;
            } else {
                this.hasMoreOlder = false;
            }
        } catch (error) {
            console.error('Failed to load older messages:', error);
        } finally {
            this.isLoadingOlder = false;
        }
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
        this.lastAcknowledgementType = undefined;
        this.lastAcknowledgementTime = undefined;
    }

    private onMessageContent(event: TimelineEntry): void {
        this.lastAcknowledgementType = undefined;
        this.lastAcknowledgementTime = undefined;
        this.agentStatus = undefined;
        this.agentStatusTime = undefined;
    }
}
