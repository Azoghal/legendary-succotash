use rocket::tokio;
use rspotify::{
    clients::OAuthClient,
    model::{AdditionalType, Country, Market, PlayableItem},
    scopes, AuthCodeSpotify, ClientCredsSpotify, Config, Credentials, OAuth,
};

use crate::errors;

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

        UserSpotifyApi {
            auth_code: AuthCodeSpotify::with_config(creds, oauth, Config::default()),
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

    pub async fn get_the_token(&self, code: &str) -> Result<(), errors::Error> {
        // think we should request a refresh token as well, whack that in the db,
        // and then we can load it and request a token lazily when needed on each request.
        self.auth_code.request_token(code).await?;
        let token_opt = self.auth_code.read_token_cache(true).await?;
        if let Some(token) = token_opt {
            info!("the token's refresh code {:?}", token.refresh_token);
        }
        // And here we would write the token to the DB
        // Can we make a new token just from the refresh token or must i actually db-ify the entire token?
        Ok(())
    }

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
