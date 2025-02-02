import type { User } from "$lib/models/user";

const sampleResponse: User = {
    id: "sdljafhlvwn",
    username: 'norpie',
    email: 'contact@norpie.dev',
    avatar: 'https://avatars.githubusercontent.com/u/46564751?v=4'
};

export async function fetchUser(): Promise<User> {
    return sampleResponse;
}
