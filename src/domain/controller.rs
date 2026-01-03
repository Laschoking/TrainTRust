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

pub struct Controller {
    mongo: MongoClient,
    vendo_socket: VendoSocket,
    /// Contains all trips that are stored in Database
    trips: Vec<Trip>,
    /// Contains all stations stored in the Database
    stations: Stations,
}

impl Controller {
    pub async fn try_new() -> Result<Self, ConnectionError> {
        let mongo = MongoClient::try_connect(MONGO_CONNECTION_STRING).await?;
        let vendo_socket = VendoSocket::try_from(VENDO_URI)?;
        let stations = mongo.load_stations().await?;
        // TODO Potentially at some point we dont want to load everything anymore, but only some filtered data
        let trips = mongo.load_trips().await?;
        let profiles = mongo.load_profiles().await?;

        Ok(Self {
            mongo,
            vendo_socket,
            trips,
            stations,
        })
    }

    pub async fn update_trips(&self) -> Result<(), ConnectionError> {
        todo!()
    }

    /// Forward request to Mongo Client
    pub async fn add_user<'a>(
        &self,
        user: &'a mut BahnProfile,
    ) -> Result<&'a BahnProfile, ConnectionError> {
        self.mongo.add_user(user).await
    }

    pub async fn new_trips(
        user: &str,
        origin: &str,
        destination: &str,
        date: DateTime<FixedOffset>,
    ) -> Result<(), ConnectionError> {
        todo!()
    }
}
