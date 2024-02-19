
import { Popularity } from "../gen/types/Popularity";

class SpotifyExampleClient {
    async get(artist_id:string): Promise<Popularity> {
        // TODO get base url from env or context
        const baseUrl = "http://127.0.0.1:8000/api/v1"
        const route = "/artist-popularity"
        const idStr = "/" + artist_id 
        return fetch(baseUrl + route + idStr)
            .then((response) => response.json())
            .then((json) => json as Popularity);
    }
}

export function newSpotifyExampleClient(): SpotifyExampleClient {
    return new SpotifyExampleClient();
}
