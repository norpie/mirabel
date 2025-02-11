import { writable, type Writable } from 'svelte/store';

import type { User } from '$lib/models/user';
import type { Chat } from './models/chat';
import type { Project } from './models/project';

export const user: Writable<User | null> = writable(null);
export const projects: Writable<Project[] | null> = writable(null);
export const selectedProject: Writable<Project | null> = writable(null);
export const chats: Writable<Chat[] | null> = writable(null);
export const selectedChat: Writable<Chat | null> = writable(null);
export const breadcrumbs: Writable<string[]> = writable([]);
