use crate::models::Popularity;

use rspotify::{clients::BaseClient, model::ArtistId, ClientCredsSpotify, Credentials};

pub async fn get_artist_popularity(id: &str) -> Result<Popularity, ()> {
    let creds = Credentials::from_env().expect("failed to get credentials from env");
    let spotify = ClientCredsSpotify::new(creds);

    spotify
        .request_token()
        .await
        .expect("failed to get bearer token");

    let artist = ArtistId::from_id("0YrtvWJMgSdVrk3SfNjTbx").expect("failed to get artist id");
    let res = spotify
        .artist(artist)
        .await
        .expect("failed to get artist from API request");

    Ok(Popularity {
        popularity: res.popularity,
    })
}
