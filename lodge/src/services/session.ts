import { User } from "../gen/types/User";
import { AuthUrl } from "../gen/types/AuthUrl";
import { BaseClient } from "./baseClient";

class SessionClient extends BaseClient{

    constructor(){
        super();
    }

    async getUser(): Promise<User>{
        const route = this.baseUrl + "/session_user"

        return new Promise((resolve, reject)=>{
            fetch(route, {credentials: "same-origin", headers: {"Content-Type":"application/json"}})
            .then((response) => { 
                if (response.ok) {
                    resolve(response.json().then((json)=>json as User))
                }
                reject([route, response.statusText].join(" "))
            })
        })
    }

    async getAuthUrl(): Promise<string>{
        const route = this.baseUrl+"/authorize_url"

        return new Promise((resolve, reject)=>{
            fetch(route, {credentials: "same-origin", headers: {"Content-Type":"application/json"}})
            .then((response) => { 
                if (response.ok) {
                    resolve(response.json().then((json)=>{const res = json as AuthUrl; return res.url}))
                }
                reject([route, response.statusText].join(" "))
            })
        })
    }
}

export function newSessionClient(): SessionClient {
    return new SessionClient();
}