use serde::{Deserialize, Serialize};

use super::{
    bahn_profiles::BahnProfile,
    client::{DocumentCollection, InsertPendingDocument},
    journeys::JourneySummary,
    stations::{Station, StationIbnr},
};
use chrono::{DateTime, FixedOffset};
use mongodb::bson::oid::ObjectId;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct PendingTrip {
    origin: StationIbnr,
    destination: StationIbnr,
    date: DateTime<FixedOffset>,
    // The unique identifier name of a [BahnProfile]
    user: String,
    journey_sum: Vec<JourneySummary>,
}

///
#[derive(Deserialize)]
pub struct Trip {
    id: ObjectId,
    origin: StationIbnr,
    destination: StationIbnr,
    date: DateTime<FixedOffset>,
    user: String,
    journey_sum: Vec<JourneySummary>,
}

pub struct Trips(Vec<Trip>);

impl DocumentCollection for Trips {
    type Document = Trip;
    fn add(&mut self, document: Self::Document) -> &mut Self::Document {
        self.0.push(document);
        self.0
            .last_mut()
            .expect("At least one value is in collection")
    }
}

impl InsertPendingDocument for PendingTrip {
    const COLLECTION: &'static str = "trips";
    type Persisted = Trip;

    fn with_id(self, id: ObjectId) -> Self::Persisted {
        Self::Persisted {
            id,
            origin: self.origin,
            destination: self.destination,
            date: self.date,
            user: self.user,
            journey_sum: self.journey_sum,
        }
    }
}

/// Contains a compressed view on travelling connections based on the parameters
/// origin, destination, date, user_profile
impl PendingTrip {
    /// Construct a new trip
    pub fn new(
        user: String,
        origin: StationIbnr,
        destination: StationIbnr,
        date: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            origin,
            destination,
            date,
            user,
            journey_sum: Vec::new(),
        }
    }
}

impl Trip {
    /// Extract parameters for the API request
    pub fn http_params(&self) -> HashMap<String, String> {
        HashMap::from([
            (String::from("from"), self.origin.to_string()),
            (String::from("to"), self.destination.to_string()),
            (String::from("date"), self.date.to_string()),
        ])
    }

    /// Update a the [JourneySummary] of a [Trip]
    pub fn update(&mut self, trip: &Trip) {
        // TODO Update the JourneySUmmary here:
        todo!()
    }
}

impl Trips {
    /// Initiate an empty [Trips] collection
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Finds an existing trip for specified parameters
    pub fn find_mut(
        &mut self,
        user_name: String,
        origin: &StationIbnr,
        destination: &StationIbnr,
        date: DateTime<FixedOffset>,
    ) -> Option<&mut Trip> {
        self.0.iter_mut().find(|trip| {
            trip.user == user_name
                && trip.origin == *origin
                && trip.destination == *destination
                && trip.date == date
        })
    }
}

impl From<Vec<Trip>> for Trips {
    fn from(value: Vec<Trip>) -> Self {
        Self(value)
    }
}
