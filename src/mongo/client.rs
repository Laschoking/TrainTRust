//! Provide connection to MongoDB and document manipulation options.
use super::{
    deutsche_bahn::BahnProfile,
    stations::{Station, Stations},
    trip::Trip,
};
/// TODO: Maybe the implementation of IO should be handled by the MongoClient
use crate::errors::ConnectionError;
use mongodb::{
    Client, Collection, Database,
    bson::{Bson, Document, doc, oid::ObjectId},
};

use std::collections::HashSet;
use std::sync::Arc;
use tokio::net::TcpStream;
use unidecode::unidecode;
use url::Url;

pub struct MongoClient {
    /// Entry point of the 'train_tracker' database
    database: Database,
}
impl MongoClient {
    /// Creates a client for with connection to MongoDB server
    pub async fn try_connect(uri: &str) -> Result<Self, ConnectionError> {
        // Validate uri before making connection attempt
        let url = Url::parse(uri)?;
        let addr = format!(
            "{}:{}",
            url.domain().expect("Domain should not be empty"),
            url.port().expect("Port should not be empty")
        );
        TcpStream::connect(addr).await?;
        let client = mongodb::Client::with_uri_str(url.as_str()).await?;
        let database = client.database("train_tracker");
        Ok(Self { database })
    }

    /// Return a reference to the database
    pub fn database(&self) -> &Database {
        &self.database
    }
    // TODO: eigenen Trait implementieren?
    // Load: stations/ bahn_profiles/ trips/ journeys
    // Update: trips
    // Insert: bahn_profiles/ trips/ journeys

    pub async fn load_stations(&self) -> Result<Stations, ConnectionError> {
        let mut cursor: mongodb::Cursor<Station> = self
            .database
            .collection("stations")
            .find(doc! {})
            .projection(doc! {"_id": 1, "Name": 1, "IBNR" :1})
            .await?;
        let mut stations = HashSet::new();
        while cursor.advance().await? {
            let mut station = cursor.deserialize_current()?;
            station.name = unidecode(&station.name);
            stations.insert(station);
        }
        Ok(Stations(stations))
    }

    pub async fn load_profiles(&self) -> Result<Vec<BahnProfile>, ConnectionError> {
        todo!()
    }

    pub async fn load_trips(&self) -> Result<Vec<Trip>, ConnectionError> {
        todo!()
    }

    /// Insert new [BahnProfile] in MongoDB collections
    /// and overwrite its [ObjectId]
    pub async fn add_user<'a>(
        &self,
        user: &'a mut BahnProfile,
    ) -> Result<&'a BahnProfile, ConnectionError> {
        if let Some(id) = self
            .database
            .collection("bahn_profiles")
            .find_one(doc! {"user_name": user.name()})
            .projection(doc! {"_id" : 1})
            .await?
        {
            user.id = Some(id);
        } else {
            let result = self
                .database
                .collection::<BahnProfile>("bahn_profiles")
                .insert_one(&mut *user)
                .await?;
            user.id = result.inserted_id.as_object_id();
        }
        Ok(user)
    }

    /// Remove a [BahnProfile] by its ObjectId
    pub async fn drop_user(&self, id: ObjectId) -> Result<(), ConnectionError> {
        self.database
            .collection::<BahnProfile>("bahn_profiles")
            .delete_one(doc! {"_id": id})
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MongoClient;
    use crate::{config::MONGO_CONNECTION_STRING, errors::ConnectionError};
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
    async fn valid_uri() -> Result<(), ConnectionError> {
        let client = MongoClient::try_connect(MONGO_CONNECTION_STRING).await?;
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
