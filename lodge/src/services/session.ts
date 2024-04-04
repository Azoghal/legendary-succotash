import { User } from "../gen/types/User";

class SessionClient {
    async getUser(): Promise<User>{
        // TODO remove
        const baseUrl = process.env.REACT_APP_BASE_API_URL
        const route = "/session_user"

        return new Promise((resolve, reject)=>{
            fetch(baseUrl + route, {credentials: "same-origin", headers: {"Content-Type":"application/json"}})
            .then((response) => { 
                if (response.ok) {
                    resolve(response.json().then((json)=>json as User))
                }
                reject([baseUrl+route, response.statusText].join(" "))
            })
        })
    }
}

export function newSessionClient(): SessionClient {
    return new SessionClient();
}