
import { Popularity } from "../gen/types/Popularity";

class SpotifyExampleClient {
    async get(artist_id:string): Promise<Popularity> {
        // TODO get base url from env or context
        const baseUrl = "http://localhost:8000/api/v1"
        const route = "/artist-popularity"
        const idStr = "/" + artist_id 
        return fetch(baseUrl + route + idStr)
            .then((response) => response.json())
            .then((json) => json as Popularity);
    }

    async session_test(): Promise<void>{
        // TODO remove
        const baseUrl = "http://localhost:8000/api/v1"
        const route = "/user-session-test"
        // TODO this actually succeeds even if it's an error code
        return fetch(baseUrl + route, {credentials: "same-origin"})
            .then(() => console.log("Success")).catch(()=>console.log("err"))
    }
}

export function newSpotifyExampleClient(): SpotifyExampleClient {
    return new SpotifyExampleClient();
}
