import { writable, type Writable } from 'svelte/store';

import type { User } from '$lib/models/user';
import type { Chat } from './models/chat';
import type { Workspace } from './models/workspace';

export const user: Writable<User | null> = writable(null);
export const workspaces: Writable<Workspace[] | null> = writable(null);
export const selectedWorkspace: Writable<Workspace | null> = writable(null);
export const chats: Writable<Chat[] | null> = writable(null);
export const selectedChat: Writable<Chat | null> = writable(null);
export const breadcrumbs: Writable<string[]> = writable([]);
