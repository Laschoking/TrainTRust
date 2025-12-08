//! Provide connection to MongoDB and document manipulation options.
use crate::errors::ConnectionError;
use mongodb::{
    bson::{Document, doc},
    sync::{Client, Collection, Database},
};

/// Make a connection to a local MongoDB server
pub fn mongo_client(uri: &str) -> Result<Database, ConnectionError> {
    let client = Client::with_uri_str(uri)?;

    let database = client.database("train_tracker");

    Ok(database)
}

#[cfg(test)]
mod tests {
    use super::mongo_client;
    use crate::errors::ConnectionError;

    #[test]
    fn test_uri() -> Result<(), ConnectionError> {
        let uri = "mongodb://root:example@localhost:27017/?authSource=admin";
        assert!(mongo_client(uri).is_ok());
        Ok(())
    }

    #[test]
    fn test_bad_uri() {
        let uri = "mongodb";
        assert!(mongo_client(uri).is_err());
    }
}
