//! Provide connection to MongoDB and document manipulation options.
use super::{
    bahn_profiles::{BahnProfile, BahnProfiles, PendingBahnProfile},
    journeys::{Journey, JourneySummary, Journeys, PendingJourney},
    stations::{Station, Stations},
    trips::{PendingTrip, Trip, Trips},
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

/// Specify the type of [MongoDocument] a [DocumentCollection] will contain
pub trait DocumentCollection {
    type Document: MongoDocument;
    fn add(&mut self, document: Self::Document) -> &mut Self::Document;
}

/// Specify the MongoDB collection string for each document type
pub trait MongoDocument: Sized + for<'de> Deserialize<'de> + Unpin + Send + Sync {
    const COLLECTION: &'static str;
    fn id(&self) -> &ObjectId;
}

pub trait InsertPendingDocument: Sized + Serialize + Send + Sync {
    const COLLECTION: &'static str;
    type Persisted: MongoDocument;
    fn with_id(self, id: ObjectId) -> Self::Persisted;
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
    pub async fn load<C>(&self) -> Result<C, ConnectionError>
    where
        C: DocumentCollection + From<Vec<C::Document>>,
    {
        let cursor = self
            .database
            .collection::<C::Document>(C::Document::COLLECTION)
            .find(doc! {})
            .await?;

        let vec = cursor.try_collect().await?;
        Ok(C::from(vec))
    }

    /// Delete a document by its [ObjectId] from collection
    pub async fn drop<T>(&self, id: &ObjectId) -> Result<(), ConnectionError>
    where
        T: MongoDocument + Send + Sync,
    {
        self.database
            .collection::<T>(T::COLLECTION)
            .delete_one(doc! {"_id": id})
            .await?;
        Ok(())
    }

    pub async fn push_trip_summary(
        &self,
        trip: &mut Trip,
        summary: JourneySummary,
    ) -> Result<(), ConnectionError> {
        self.database
            .collection::<Trip>(Trip::COLLECTION)
            .update_one(
                doc! {"_id" : trip.id()},
                doc! {"$push" :
                {"journey_summary" :  mongodb::bson::to_bson(&summary)?}},
            )
            .await?;

        trip.add_summary(summary);
        Ok(())
    }

    /// Push a pending document to MongoDB and safe it with [ObjectId] in collection
    pub async fn persist_and_add<'a, R, T>(
        &self,
        collection: &'a mut R,
        pending: T,
    ) -> Result<&'a mut R::Document, ConnectionError>
    where
        R: DocumentCollection,
        T: InsertPendingDocument<Persisted = R::Document>,
    {
        let result = self
            .database
            .collection::<T>(T::COLLECTION)
            .insert_one(&pending)
            .await?;
        let id = result
            .inserted_id
            .as_object_id()
            .expect("This should return an ObjectId");
        let value = pending.with_id(id);
        Ok(collection.add(value))
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
