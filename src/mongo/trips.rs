use serde::{Deserialize, Serialize};

use super::{
    client::{DocumentCollection, InsertPendingDocument, MongoDocument},
    journeys::JourneySummary,
    stations::StationIbnr,
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
    journey_summary: Vec<JourneySummary>,
}

///
#[derive(Deserialize)]
pub struct Trip {
    #[serde(rename = "_id")]
    id: ObjectId,
    origin: StationIbnr,
    destination: StationIbnr,
    date: DateTime<FixedOffset>,
    user: String,
    journey_summary: Vec<JourneySummary>,
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

impl MongoDocument for Trip {
    const COLLECTION: &'static str = "trips";
    fn id(&self) -> &ObjectId {
        &self.id
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
            journey_summary: self.journey_summary,
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
            journey_summary: Vec::new(),
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

    /// Insert a new [JourneySummary] to a [Trip]
    pub fn add_summary(&mut self, summary: JourneySummary) {
        self.journey_summary.push(summary);
    }

    /// Return the user name of a profile
    pub fn user(&self) -> String {
        self.user.clone()
    }
}

impl Trips {
    /// Initiate an empty [Trips] collection
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Trip> {
        self.0.iter_mut()
    }

    /// Finds an existing trip for specified parameters
    pub fn find(
        &self,
        user_name: &String,
        origin: &StationIbnr,
        destination: &StationIbnr,
        date: DateTime<FixedOffset>,
    ) -> Option<&Trip> {
        self.0.iter().find(|trip| {
            trip.user == *user_name
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
