use rocket::{http::Status, tokio};
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{AdditionalType, Country, Market, PlayableItem},
    scopes, AuthCodeSpotify, ClientCredsSpotify, Config, Credentials, OAuth, Token,
};

use crate::{errors, routes::auth0::SessionUser, services::spotify_tokens, SuccDb};

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
    pub async fn load_user_token(db: &SuccDb, user_id: i32) -> Result<Token, errors::Error> {
        let loaded_token = spotify_tokens::get_user_token(db, user_id).await?;
        match loaded_token {
            Some(tok) => {
                let token_text = &tok.token;
                let token: Token = rocket::serde::json::from_str(token_text)?;
                Ok(token)
            }
            None => Err(errors::Error::NotFound(
                "user access token not found".into(),
            )),
        }
    }

    pub async fn get_current_playing(&self) -> Result<Option<String>, errors::Error> {
        info!("Called get_current_playing");
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
            PlayableItem::Track(t) => Ok(Some(t.name)),
            PlayableItem::Episode(e) => Ok(Some(e.name)),
        }
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for UserSpotifyApi {
    type Error = ();

    async fn from_request(
        request: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<UserSpotifyApi, ()> {
        use rocket::request::Outcome;

        let Outcome::Success(user) = SessionUser::from_request(request).await else {
            error!("failed to get user from request");
            return Outcome::Error((Status::NotFound, ()));
        };

        let db = match SuccDb::from_request(request).await {
            Outcome::Success(db) => db,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(s) => return Outcome::Forward(s),
        };

        let tok = UserSpotifyApi::load_user_token(&db, user.id).await;
        match tok {
            Ok(token) => {
                let Some(creds) = Credentials::from_env() else {
                    error!("failed to get credentials from env");
                    return Outcome::Error((Status::InternalServerError, ()));
                };

                let Some(oauth) =
                    OAuth::from_env(scopes!("user-read-currently-playing", "user-top-read"))
                else {
                    error!("failed to create scopes");
                    return Outcome::Error((Status::InternalServerError, ()));
                };

                let conf = Config {
                    token_refreshing: true,
                    ..Default::default()
                };

                let mut auth_code = AuthCodeSpotify::from_token(token);
                auth_code.creds = creds;
                auth_code.oauth = oauth;
                auth_code.config = conf;

                // let res = auth_code.auto_reauth().await;
                // if let Err(e) = res {
                //     error!("failed to auto refresh {e}")
                // };

                let spotify = UserSpotifyApi { auth_code };

                Outcome::Success(spotify)
            }
            Err(e) => {
                // TODO we can rework this to redirect to login, but can't be bothered to do this yet.
                error!("failed to get user token {}", e);
                Outcome::Error((Status::NotFound, ()))
            }
        }
    }
}

pub struct UserSpotifyHelper {
    auth_code: AuthCodeSpotify,
}

impl UserSpotifyHelper {
    pub fn new() -> Self {
        let creds = Credentials::from_env().expect("failed to get credentials from env");

        let oauth = OAuth::from_env(scopes!("user-read-currently-playing", "user-top-read"))
            .expect("oh no");

        let conf = Config {
            ..Default::default()
        };

        UserSpotifyHelper {
            auth_code: AuthCodeSpotify::with_config(creds, oauth, conf),
        }
    }

    pub async fn get_spotify_auth_url(&self) -> Result<String, errors::Error> {
        self.auth_code.get_authorize_url(true).map_err(|e| e.into())
    }

    // get_new_user_token requests a token for the current user, and attempts to store it in the DB
    // it only returns an error in the case that fetching the code fails.
    // failure to write to the DB is ignored.
    pub async fn get_new_user_token(
        &self,
        db: &SuccDb,
        user_id: i32,
        code: &str,
    ) -> Result<(), errors::Error> {
        self.auth_code.request_token(code).await?;

        let _ = self.save_user_token(db, user_id).await;
        Ok(())
    }

    // save_user_token reads the token from the cache and stores it in the database
    async fn save_user_token(&self, db: &SuccDb, user_id: i32) -> Result<(), errors::Error> {
        let token_mutex = self.auth_code.get_token();
        let token_lock_acquired = token_mutex.lock().await.unwrap();

        match (*token_lock_acquired).clone() {
            None => Err(errors::Error::NotFound("refresh token not found".into())),
            Some(token) => {
                // it's serializable, so serialize it and then get it as a json string.
                let token_as_str = rocket::serde::json::to_string(&token);
                match token_as_str {
                    Err(e) => {
                        error!("failed to convert token json to string");
                        Err(e.into())
                    }
                    Ok(s) => {
                        spotify_tokens::create_spotify_token(db, user_id, s).await?;
                        Ok(())
                    }
                }
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for UserSpotifyHelper {
    type Error = ();
    async fn from_request(
        _: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<UserSpotifyHelper, ()> {
        rocket::request::Outcome::Success(Self::new())
    }
}
