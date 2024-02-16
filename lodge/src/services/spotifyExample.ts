// TODO gotta make a response type from rust that wraps the number in a json object

class SpotifyExampleClient {
    async get(artist_id:string): Promise<number> {
        // TDOD get base url from env or context
        const baseUrl = "http://127.0.0.1:8000/api/v1"
        const route = "/artist-popularity"
        const idStr = "/" + artist_id 
        return fetch(baseUrl + route + idStr)
            .then((response) => response.json())
            .then((json) => json);
    }
}

export function newSpotifyExampleClient(): SpotifyExampleClient {
    return new SpotifyExampleClient();
}
