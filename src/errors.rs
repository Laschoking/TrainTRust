//use mongodb::error::Error;
use thiserror::Error;
use tokio::task::JoinError;

use crate::vendo_socket::VendoError;

#[derive(Error, Debug)]
pub enum ConnectionError {
    ///
    #[error(transparent)]
    MongoConnectionFailure(#[from] mongodb::error::Error),
    ///
    #[error("Mongo Server is down")]
    MongoServerDown,
    #[error(transparent)]
    VendoError(#[from] VendoError),
    /// Error due to mismapping btw user station name and database
    #[error("Station name is invalid {name}")]
    InvalidStation { name: String },
    ///
    #[error(transparent)]
    TokioJoin(#[from] JoinError),
    /// Error due failed connection to uri
    #[error(transparent)]
    TcpError(#[from] std::io::Error),
    /// Error due to invalid uri while parsing
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
}
