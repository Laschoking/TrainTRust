//! Provide connection to MongoDB and document manipulation options.
use crate::errors::ConnectionError;
use mongodb::{
    Client, Collection, Database,
    bson::{Document, doc},
};
use std::sync::Arc;
use tokio::net::TcpStream;
use url::Url;

pub struct MongoClient {
    /// Entry point of the 'train_tracker' database
    database: Database,
}
impl MongoClient {
    /// Creates a client for with connection to MongoDB server
    pub async fn try_connect(uri: &str) -> Result<Arc<Self>, ConnectionError> {
        // Validate uri before making connection attemp
        let url = Url::parse(uri)?;
        let addr = format!(
            "{}:{}",
            url.domain().expect("Domain should not be empty"),
            url.port().expect("Port should not be empty")
        );
        TcpStream::connect(addr).await?;
        let client = Client::with_uri_str(url.as_str()).await?;
        let database = client.database("train_tracker");

        let client = Self { database };
        Ok(Arc::new(client))
    }

    /// Return a reference to the database
    pub fn database(&self) -> &Database {
        &self.database
    }
}

#[cfg(test)]
mod tests {
    use super::MongoClient;
    use crate::errors::ConnectionError;
    use futures::stream::{StreamExt, TryStreamExt};
    use mongodb::bson::{Bson, doc};

    #[tokio::test]
    async fn ping_invalid_server() {
        let uri = "mongodb://root:example@localhost:27018/?authSource=admin";
        if let Err(ConnectionError::TcpError(e)) = MongoClient::try_connect(uri).await {
            println!("TcpError: {e:?}");
        } else {
            panic!("Unexpected Error result");
        }
    }

    #[tokio::test]
    async fn test_uri() -> Result<(), ConnectionError> {
        let uri = "mongodb://root:example@localhost:27017/?authSource=admin";
        let client = MongoClient::try_connect(uri).await?;
        let count = client
            .database()
            .collection::<Bson>("stations")
            .count_documents(doc! {})
            .await?;
        if count > 0 {
            println!("Count of stations: {count}");
            Ok(())
        } else {
            Err(ConnectionError::MongoServerDown)
        }
    }

    #[tokio::test]
    async fn invalid_uri() {
        let uri = "mongodb";
        if let Err(ConnectionError::UrlParseError(e)) = MongoClient::try_connect(uri).await {
            println!("ParseError: {e:?}");
        } else {
            panic!("Unexpected UrlParse result");
        }
    }
}
