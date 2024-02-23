use crate::{models::Popularity, SpotifyApi};

use rspotify::{clients::BaseClient, model::ArtistId};

pub async fn get_artist_popularity(id: &str, spotify: &SpotifyApi) -> Result<Popularity, ()> {
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
