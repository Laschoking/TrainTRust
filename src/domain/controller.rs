use crate::{
    config::{MONGO_CONNECTION_STRING, VENDO_URI},
    errors::ConnectionError,
    mongo::{
        client::MongoClient, deutsche_bahn::BahnProfile, journeys::JourneySummary,
        stations::Stations, trip::Trip,
    },
    vendo::{client::VendoSocket, journeys::JourneyRequest},
};
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Orchestrates the API client and the data flow in/out of MongoDB database
pub struct Controller {
    /// Handles document manipulation for MongoDB
    db_client: MongoClient,
    /// Handles connection and requests to the vendo API
    vendo_socket: VendoSocket,
    /// All trips stored in MongoDB
    trips: Vec<Trip>,
    /// Unique stations in MongoDB (retrieved from Wikidata)
    stations: Stations,
    /// All [BahnProfile]s that are stored in MongoDB
    profiles: Vec<BahnProfile>,
}

impl Controller {
    /// Initiate connection to [VendoSocket] and load trip data from [MongoClient]
    pub async fn try_new() -> Result<Self, ConnectionError> {
        let db_client = MongoClient::try_connect(MONGO_CONNECTION_STRING).await?;
        let vendo_socket = VendoSocket::try_from(VENDO_URI)?;
        let stations = db_client.load_stations().await?;
        // TODO Potentially at some point we dont want to load everything anymore, but only some filtered data
        let trips = db_client.load_trips().await?;
        let profiles = db_client.load_profiles().await?;

        Ok(Self {
            db_client,
            vendo_socket,
            trips,
            stations,
            profiles,
        })
    }

    pub async fn update_trips(&self) -> Result<(), ConnectionError> {
        todo!()
    }

    /// Add new user with [BahnProfile] to MongoDB
    pub async fn add_user<'a>(
        &self,
        user: &'a mut BahnProfile,
    ) -> Result<&'a BahnProfile, ConnectionError> {
        self.db_client.add_user(user).await
    }

    /// Retrieve new journeys from Vendo API
    pub async fn new_trips(
        user: &str,
        origin: &str,
        destination: &str,
        date: DateTime<FixedOffset>,
    ) -> Result<(), ConnectionError> {
        todo!()
    }
}
