use rocket::tokio;
use rspotify::{scopes, AuthCodeSpotify, ClientCredsSpotify, Config, Credentials, OAuth};

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

pub struct UserSpotifyApi {
    pub auth_code: AuthCodeSpotify,
}

impl UserSpotifyApi {
    #[rocket::tokio::main]
    pub async fn new() -> Self {
        let creds = Credentials::from_env().expect("failed to get credentials from env");

        let oauth = OAuth {
            scopes: scopes!("user-read-currently-playing", "user-top-read"),
            redirect_uri: "http://localhost:8000/callback".to_owned(),
            ..Default::default()
        };

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
}
