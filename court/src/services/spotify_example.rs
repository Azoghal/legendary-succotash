use crate::{models::Popularity, SpotifyApi};

use rocket::State;
use rspotify::{clients::BaseClient, model::ArtistId};

pub async fn get_artist_popularity(
    id: &str,
    spotify: &State<SpotifyApi>,
) -> Result<Popularity, ()> {
    // let creds = Credentials::from_env().expect("failed to get credentials from env");
    // let spotify = ClientCredsSpotify::new(creds);

    spotify
        .client
        .request_token()
        .await
        .expect("failed to get bearer token");

    let artist = ArtistId::from_id(id).expect("failed to get artist id");
    let res = spotify
        .client
        .artist(artist)
        .await
        .expect("failed to get artist from API request");

    Ok(Popularity {
        popularity: res.popularity,
    })
}
