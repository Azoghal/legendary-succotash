import { User } from "../gen/types/User";

class SessionClient {
    async getUser(): Promise<User>{
        // TODO remove
        const baseUrl = "http://localhost:8000/api/v1"
        const route = "/user-session-test"
        const json_resp = fetch(baseUrl + route, {credentials: "same-origin"})
            .then((response) => response.ok? response.json() : {})
        return json_resp.then((json)=>json as User)
    }
}

export function newSessionClient(): SessionClient {
    return new SessionClient();
}