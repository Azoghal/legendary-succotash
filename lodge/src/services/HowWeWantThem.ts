import { BaseClient } from "./baseClient"
import * as rTypes from "../gen/types/RustTypes"


export async function get_artist_popularity(baseClient: BaseClient, bob: number): Promise<rTypes.Popularity> {
    const route = baseClient.baseUrl + "/artist-popularity";
    console.log(bob);
    return fetch(route)
        .then((response) => { return response.json()})
        .then((json) => json as rTypes.Popularity);
}


export async function get_current_playing(baseClient: BaseClient, bob: number): Promise<rTypes.CurrentPlaying> {
    const route = baseClient.baseUrl + "/currently-playing";
    console.log(bob);
    return fetch(route)
        .then((response) => { return response.json()})
        .then((json) => json as rTypes.CurrentPlaying);
}
