//use mongodb::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    MongoConnectionFailure(#[from] mongodb::error::Error),
}
