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

pub struct UserSpotifyApi {
    pub auth_code: AuthCodeSpotify,
}

impl UserSpotifyApi {
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
}
