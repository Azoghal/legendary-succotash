use rocket::{
    http::Status,
    response::{self, Responder},
    Request,
};
use rspotify::model::IdError;
use rspotify_http;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("SpotifyId Error {source:?}")]
    SpotifyId {
        #[from]
        source: IdError,
    },

    #[error("SpotifyClientError {source:?}")]
    SpotifyClient {
        #[from]
        source: rspotify::ClientError,
    },

    #[error("DB Error {source:?}")]
    DbError {
        #[from]
        source: diesel::result::Error,
    },

    #[error("jsonwebtoken error {source:?}")]
    JWTError {
        #[from]
        source: jsonwebtoken::errors::Error,
    },

    #[error("currently no sensible succotash error for: `{0}`")]
    Placeholder(String),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);

        fn map_spotify_client_http_error<'a>(
            err: rspotify_http::HttpError,
            req: &Request,
        ) -> Result<rocket::Response<'a>, Status> {
            match err {
                rspotify_http::HttpError::Client(e) => {
                    error!("{e:?}");
                    // TODO match here for more granularity?
                    Status::BadRequest.respond_to(req)
                }
                rspotify_http::HttpError::StatusCode(response) => {
                    error!("{response:?}");
                    let code = response.status().as_u16();
                    // TODO extend these cases
                    match code {
                        400 => Status::BadRequest.respond_to(req),
                        _ => Status::InternalServerError.respond_to(req),
                    }
                }
            }
        }

        match self {
            Error::SpotifyId { source } => {
                error!("{source:?}");
                Status::BadRequest.respond_to(req)
            }
            Error::SpotifyClient { source } => match source {
                rspotify::ClientError::Http(error) => map_spotify_client_http_error(*error, req),
                _ => {
                    error!("{source:?}");
                    Status::InternalServerError.respond_to(req)
                }
            },
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
