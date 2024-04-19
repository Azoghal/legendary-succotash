use rocket::tokio;
use rspotify::{
    clients::OAuthClient,
    model::{AdditionalType, Country, Market, PlayableItem},
    scopes, AuthCodeSpotify, ClientCredsSpotify, Config, Credentials, OAuth,
};

use crate::errors;

use super::users;

pub struct SpotifyApi {
    pub client: ClientCredsSpotify,
}

impl SpotifyApi {
    // use macro to make blocking so we can just call it easily on server startup.
    #[rocket::tokio::main]
    pub async fn new() -> Self {
        let creds = Credentials::from_env().expect("failed to get credentials from env");
        let client = ClientCredsSpotify::new(creds);

        client
            .request_token()
            .await
            .expect("failed to get spotify token");

        SpotifyApi { client }
    }
}

/// 0. Generate a request URL with [`Self::get_authorize_url`].
/// 1. The user logs in with the request URL. They will be redirected to the
///    given redirect URI, including a code in the URL parameters. This happens
///    on your side.
/// 2. The code obtained in the previous step is parsed with
///    [`Self::parse_response_code`].
/// 3. The code is sent to Spotify in order to obtain an access token with
///    [`Self::request_token`].
/// 4. Finally, this access token can be used internally for the requests.
///    It may expire relatively soon, so it can be refreshed with the refresh
///    token (obtained in the previous step as well) using
///    [`Self::refresh_token`]. Otherwise, a new access token may be generated
///    from scratch by repeating these steps, but the advantage of refreshing it
///    is that this doesn't require the user to log in, and that it's a simpler
///    procedure.

// TODO need to give this a request guard so we can build it from session user rather than having one global one
pub struct UserSpotifyApi {
    pub auth_code: AuthCodeSpotify,
}

impl UserSpotifyApi {
    #[rocket::tokio::main]
    pub async fn new() -> Self {
        let creds = Credentials::from_env().expect("failed to get credentials from env");

        let oauth = OAuth::from_env(scopes!("user-read-currently-playing", "user-top-read"))
            .expect("oh no");

        let conf = Config {
            token_cached: true,

            ..Default::default()
        };
        // TODO might need to have user specific cache paths? it's a bit yucky

        UserSpotifyApi {
            auth_code: AuthCodeSpotify::with_config(creds, oauth, conf),
        }
    }

    pub fn have_a_go(&self) -> String {
        let bob_url = self
            .auth_code
            .get_authorize_url(true)
            .expect("failed to get authorize url");

        info!("the bob url {}", bob_url);

        bob_url
    }

    // get_the_token requests a token for the current user, and attempts to store it in the DB
    // it only returns an error in the case that fetching the code fails.
    // failure to write to the DB is ignored.
    pub async fn get_the_token(&self, user_id: i32, code: &str) -> Result<(), errors::Error> {
        self.auth_code.request_token(code).await?;

        let _ = self.save_user_token(user_id).await;
        Ok(())
    }

    // save_user_token reads the token from the cache and stores it in the database
    pub async fn save_user_token(&self, user_id: i32) -> Result<(), errors::Error> {
        let token_opt = match self.auth_code.read_token_cache(true).await {
            Ok(tok) => tok,
            Err(e) => {
                error!("oh no, the token could not be read {:?}", e);
                return Err(e.into());
            }
        };
        match token_opt {
            Some(token) => {
                // And here we would write the token to the DB
                // Can we make a new token just from the refresh token or must i actually db-ify the entire token?
                info!("the token's refresh code {:?}", token.refresh_token);
                users::set_user_refresh_token(user_id, token.refresh_token)?;
                Ok(())
            }
            None => {
                error!("got None as token_opt");
                Err(errors::Error::NotFound("refresh token not found".into()))
            }
        }
    }

    // TODO move this somewhere else?
    pub async fn get_current_playing(&self) -> Result<Option<String>, errors::Error> {
        let market = Market::Country(Country::UnitedKingdom);
        let additional_types = [AdditionalType::Track, AdditionalType::Episode];

        let res = self
            .auth_code
            .current_playing(Some(market), Some(&additional_types))
            .await?;
        let Some(r) = res else {
            info!("no currently playing");
            return Ok(None);
        };

        let Some(i) = r.item else {
            info!("no current item");
            return Ok(None);
        };

        match i {
            PlayableItem::Track(t) => {
                info!("yer playing this song: {}", t.name);
                Ok(Some(t.name))
            }
            PlayableItem::Episode(e) => {
                info!("yer listening to this episdoe: {}", e.name);
                Ok(Some(e.name))
            }
        }
    }
}
