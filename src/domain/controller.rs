use crate::{
    config::{MONGO_CONNECTION_STRING, VENDO_URI},
    errors::ConnectionError,
    mongo::{
        bahn_profiles::BahnProfile,
        client::MongoClient,
        journeys::{Journey, JourneySummary, Journeys},
        stations::{Station, Stations},
        trips::{StationIbnr, Trip, Trips},
    },
    vendo::{client::VendoSocket, journeys::JsonRequest},
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
    trips: Trips,
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
        let stations = Stations::from(db_client.load::<Station>().await?);
        // TODO Potentially at some point we dont want to load everything anymore, but only some filtered data
        let trips = Trips::from(db_client.load::<Trip>().await?);
        let profiles = db_client.load::<BahnProfile>().await?;

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

    /// Insert new user with [BahnProfile] to MongoDB
    pub async fn insert_user<'a>(
        &self,
        user: &'a mut BahnProfile,
    ) -> Result<&'a BahnProfile, ConnectionError> {
        self.db_client.insert_user(user).await
    }

    /// Updates an existing trip if user parameters match, otherwise creates a new [Trip]
    pub async fn update_trip(
        &mut self,
        user: &BahnProfile,
        origin: &str,
        destination: &str,
        date: DateTime<FixedOffset>,
    ) -> Result<(), ConnectionError> {
        let origin = self.stations.try_get(origin)?.into();
        let destination = self.stations.try_get(destination)?.into();

        let user_name = user.name().clone();
        let mut trip = match self
            .trips
            .find(user_name.clone(), &origin, &destination, date)
        {
            Some(trip) => trip,
            None => self
                .trips
                .add(Trip::new(user_name, origin, destination, date)),
        };
        let mut params = user.as_hashmap();
        params.extend(trip.http_params());
        let json = self.vendo_socket.request(params).await?;
        let des_data: JsonRequest = serde_json::from_str(json.as_str())?;
        let journeys = Journeys::from(des_data);

        println!("{journeys:?}");
        // Transform into Journey Data

        // TODO: At the end we need to save the new trip/ update the old one

        Ok(())
    }
}
