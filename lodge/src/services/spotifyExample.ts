
import { CurrentPlaying } from "../gen/types/CurrentPlaying";
import { Popularity } from "../gen/types/Popularity";
import { BaseClient } from "./baseClient";

class SpotifyExampleClient extends BaseClient {
    async getArtistPopularity(artist_id:string): Promise<Popularity> {
        const route = this.baseUrl+"/artist-popularity"+ "/" + artist_id 
        return fetch(route)
            .then((response) => response.json())
            .then((json) => json as Popularity);
    }

    async getCurrentlyPlaying(): Promise<CurrentPlaying> {
        const route = this.baseUrl + "/user/currently_playing"
        return fetch(route)
            .then((response) => {console.log("bobly bobly",response); return response.json()})
            .then((json) => json as CurrentPlaying);
    }
}

export function newSpotifyExampleClient(): SpotifyExampleClient {
    return new SpotifyExampleClient();
}
