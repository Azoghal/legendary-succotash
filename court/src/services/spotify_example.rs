use crate::{errors, models::Popularity, SpotifyApi};

use rspotify::{clients::BaseClient, model::ArtistId};

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
