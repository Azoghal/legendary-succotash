use crate::{errors, models::spotify::Popularity, spotify::SpotifyApi};

use rspotify::{clients::BaseClient, model::ArtistId};

use super::spotify::UserSpotifyApi;

pub async fn get_artist_popularity(
    id: &str,
    spotify: &SpotifyApi,
) -> Result<Popularity, errors::Error> {
    let artist = ArtistId::from_id(id)?;
    let res = spotify.client.artist(artist).await?;

    Ok(Popularity {
        popularity: res.popularity,
    })
}

pub async fn get_client_url(spotify: &UserSpotifyApi) -> Result<String, errors::Error> {
    let res = spotify.have_a_go();
    Ok(res)
}
