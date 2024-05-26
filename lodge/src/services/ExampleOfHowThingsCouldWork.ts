import { get_current_playing, get_artist_popularity } from "./generatedSpotify";
import { BaseClient } from "./baseClient";
import * as rTypes from "../gen/types/RustTypes"

class AnExampleClient extends BaseClient {
    
    constructor(){
        super();
    }

    async GetCurrentPlaying(bob: number ): Promise<rTypes.CurrentPlaying> {
        return get_current_playing(this, bob)
    } 

    async GetArtistPopularity(bob: number): Promise<rTypes.Popularity> {
        return get_artist_popularity(this, bob)
    } 
}


export function newAnExampleClient(): AnExampleClient {
    return new AnExampleClient();
}