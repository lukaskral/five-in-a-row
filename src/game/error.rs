use crate::api::fetch;
use crate::game::Game;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error<G: Game + Debug> {
    TimeoutError,
    IncorrectMove(G::Move),
    NoSuggestionAvailable,
    SuggestionComputationError,
    ApiError(fetch::Error),
    FinishedUnexpectedly,
    Invalid,
}

impl<G: Game + Debug> From<fetch::Error> for Error<G> {
    fn from(e: fetch::Error) -> Self {
        match e {
            fetch::Error::RivalTimeoutError => Self::TimeoutError,
            fetch::Error::FinishedUnexpectedly => Self::FinishedUnexpectedly,
            fetch::Error::Invalid => Self::Invalid,
            _ => Self::ApiError(e),
        }
    }
}

impl<G: Game + Debug> Display for Error<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeoutError => write!(f, "Timeout Error"),
            Self::IncorrectMove(mv) => write!(f, "The move is not valid ({:?})", mv),
            Self::NoSuggestionAvailable => write!(f, "No suggestion available"),
            Self::SuggestionComputationError => write!(f, "Suggestion computation error"),
            Self::ApiError(fetch_error) => write!(f, "Api call failed! ({:?})", fetch_error),
            Self::FinishedUnexpectedly => write!(f, "Finished unexpectedly"),
            Self::Invalid => write!(f, "Invalid request or auth"),
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
