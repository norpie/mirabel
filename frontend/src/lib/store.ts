import { writable, type Writable } from 'svelte/store';

import type { User } from '$lib/models/user';
import type { Session, ShallowSession } from './models/session';
import type { Workspace } from './models/workspace';

export const user: Writable<User | null | undefined> = writable(undefined);
export const avatar: Writable<string | null | undefined> = writable(null);
export const workspaces: Writable<Workspace[]> = writable([]);
export const selectedWorkspace: Writable<Workspace | null> = writable(null);
export const sessions: Writable<ShallowSession[] | null> = writable(null);
export const selectedSession: Writable<Session | null> = writable(null);
export const breadcrumbs: Writable<string[]> = writable([]);
