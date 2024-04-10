import { User } from "../gen/types/User";
import { AuthUrl } from "../gen/types/AuthUrl";

class SessionClient {
    async getUser(): Promise<User>{
        // TODO remove
        const baseUrl = "http://localhost:8000/api/v1"
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

    async getAuthUrl(): Promise<string>{
        // TODO remove
        const baseUrl = "http://localhost:8000/api/v1"
        const route = "/authorize_url"

        return new Promise((resolve, reject)=>{
            fetch(baseUrl + route, {credentials: "same-origin", headers: {"Content-Type":"application/json"}})
            .then((response) => { 
                if (response.ok) {
                    resolve(response.json().then((json)=>{const res = json as AuthUrl; return res.url}))
                }
                reject([baseUrl+route, response.statusText].join(" "))
            })
        })
    }
}

export function newSessionClient(): SessionClient {
    return new SessionClient();
}