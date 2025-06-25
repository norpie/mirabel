import { getContext, setContext } from "svelte";
import type { Session } from "./models/session";
import type { SocketHandler } from "./socket.svelte";
import type { SessionEvent } from "./models/event";
import type { User } from "./models/user";

export class SessionState {
    user: User | undefined = $state();
    session: Session | undefined = $state();
    socket: SocketHandler<SessionEvent> | undefined = $state();

    constructor(user: User, session: Session, socket: SocketHandler<SessionEvent>) {
        this.user = user;
        this.session = session;
        this.socket = socket;
    }
}

const SESSION_KEY = Symbol("SESSION");

export function setSessionState(user: User, session: Session, socket: SocketHandler<SessionEvent>) {
    return setContext(SESSION_KEY, new SessionState(user, session, socket));
}

export function getSessionState() {
    return getContext<ReturnType<typeof setSessionState>>(SESSION_KEY);
}
