
import { Popularity } from "../gen/types/Popularity";

class SpotifyExampleClient {
    async getArtistPopularity(artist_id:string): Promise<Popularity> {
        // TODO get base url from env or context. For localhost, things go weird if we use a mix of ip and localhost.
        const baseUrl = "http://localhost:8000/api/v1"
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
