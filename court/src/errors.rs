use rocket::{
    http::Status,
    response::{self, Responder},
    Request,
};
use rspotify::model::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("placeholder error")]
    Placeholder(String),

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
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);

        match self {
            // in our simplistic example, we're happy to respond with the default 500 responder in all cases
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
