import type Result from "$lib/models/result";
import type { User } from "$lib/models/user";
import { get } from "$lib/request";

const sampleResponse: User = {
    id: "sdljafhlvwn",
    username: 'norpie',
    email: 'contact@norpie.dev',
    avatar: 'https://avatars.githubusercontent.com/u/46564751?v=4'
};

export async function fetchUser(): Promise<Result<User | null>> {
    return await get<Result<User>>("v1/me");
}
