
import { CurrentPlaying } from "../gen/types/CurrentPlaying";
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

    async getCurrentPlaying(): Promise<CurrentPlaying> {
        // TODO get base url from env or context. For localhost, things go weird if we use a mix of ip and localhost.
        const baseUrl = "http://localhost:8000/api/v1"
        const route = "/user/current_playing"
        return fetch(baseUrl + route)
            .then((response) => response.json())
            .then((json) => json as CurrentPlaying);
    }
}

export function newSpotifyExampleClient(): SpotifyExampleClient {
    return new SpotifyExampleClient();
}
