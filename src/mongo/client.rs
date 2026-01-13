//! Provide connection to MongoDB and document manipulation options.
use super::{
    bahn_profiles::BahnProfile,
    journeys::Journey,
    stations::{Station, Stations},
    trips::{Trip, Trips},
};
/// TODO: Maybe the implementation of IO should be handled by the MongoClient
use crate::errors::ConnectionError;
use futures::TryStreamExt;
use mongodb::{
    Client, Collection, Database,
    bson::{Bson, Document, doc, oid::ObjectId},
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::sync::Arc;
use tokio::net::TcpStream;
use unidecode::unidecode;
use url::Url;

pub struct MongoClient {
    /// Entry point of the 'train_tracker' database
    database: Database,
}

pub trait MongoDocument:
    Sized + Serialize + for<'de> Deserialize<'de> + Unpin + Send + Sync
{
    const COLLECTION: &'static str;
}
impl MongoDocument for Station {
    const COLLECTION: &'static str = "stations";
}
impl MongoDocument for BahnProfile {
    const COLLECTION: &'static str = "bahn_profiles";
}
impl MongoDocument for Trip {
    const COLLECTION: &'static str = "trips";
}
impl MongoDocument for Journey {
    const COLLECTION: &'static str = "journeys";
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

    /// Return a reference to the MongoDB database
    pub fn database(&self) -> &Database {
        &self.database
    }

    /// 
    pub async fn load<T>(&self) -> Result<Vec<T>, ConnectionError>
    where
        T: MongoDocument,
    {
        let cursor = self
            .database
            .collection::<T>(T::COLLECTION)
            .find(doc! {})
            .await?;

        Ok(cursor.try_collect().await?)
    }
    
    /// Insert a document into collection
    pub async fn insert<T>(&self, t: T) -> Result<InsertOneResult, ConnectionError>
    where
        T: MongoDocument + Send + Sync,
    {
        self.database
            .collection::<T>(T::COLLECTION)
            .insert_one(t)
            .await
            .map_err(|err| err.into())
    }

    /// Delete a document by its [ObjectId] from collection
    pub async fn drop<T>(&self, id: ObjectId) -> Result<(), ConnectionError>
    where
        T: MongoDocument + Send + Sync,
    {
        self.database
            .collection::<T>(T::COLLECTION)
            .delete_one(doc! {"_id": id})
            .await?;
        Ok(())
    }

    /// Insert new [BahnProfile] in MongoDB collections
    /// Overwrites the empty [ObjectId] of the [BahnProfile] with the [ObjectId] returned from MongoDB
    pub async fn insert_user<'a>(
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
