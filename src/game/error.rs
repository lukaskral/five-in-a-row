use crate::api::fetch;
use crate::game::Game;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error<G: Game + Debug> {
    TimeoutError,
    IncorrectMove(G::Move),
    NoSuggestionAvailable,
    ApiError(fetch::Error),
}

impl<G: Game + Debug> From<fetch::Error> for Error<G> {
    fn from(e: fetch::Error) -> Self {
        if let fetch::Error::RivalTimeoutError = e {
            return Self::TimeoutError;
        }
        Self::ApiError(e)
    }
}

impl<G: Game + Debug> Display for Error<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::TimeoutError => write!(f, "Timeout Error"),
            Error::IncorrectMove(mv) => write!(f, "The move is not valid ({:?})", mv),
            Error::NoSuggestionAvailable => write!(f, "No suggestion available"),
            Error::ApiError(fetch_error) => write!(f, "Api call failed! ({:?})", fetch_error),
        }
    }
}

impl<G: Game + Debug> StdError for Error<G> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::ApiError(fetch_error) => StdError::source(fetch_error),
            _ => None,
        }
    }
}
