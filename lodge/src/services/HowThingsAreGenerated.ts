import { BaseClient } from "./baseClient"

                    import * as rTypes from "./exampleModels"

        async function get_current_playing(baseClient: BaseClient, bob: rTypes.i32): Promise<rTypes.Result < Json < CurrentPlaying >, errors :: Error >> {
            const route = baseClient.baseUrl + "/currently-playing";
            console.log(bob);
            return fetch(route)
                .then((response) => { return response.json()})
                .then((json) => json as rTypes.Result < Json < CurrentPlaying >, errors :: Error >);
        }
    

        async function get_artist_popularity(baseClient: BaseClient, bob: rTypes.i32): Promise<rTypes.Result < Json < Popularity >, errors :: Error >> {
            const route = baseClient.baseUrl + "/artist-popularity";
            console.log(bob);
            return fetch(route)
                .then((response) => { return response.json()})
                .then((json) => json as rTypes.Result < Json < Popularity >, errors :: Error >);
        }
    