//! Represents the core journeys, that can be deserialized from MongoDB or Http GET requests

use super::client::{DocumentCollection, InsertPendingDocument, MongoDocument};
use crate::vendo::journeys::{JsonJourney, JsonLeg, JsonRequest};
use chrono::Duration;
use chrono::format::Fixed;
use chrono::{DateTime, FixedOffset, Local, TimeDelta};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

///
///
#[derive(Debug)]
pub struct Journeys(Vec<Journey>);

/// This will serve as Serialization & Deserialization for MongoDB trips
#[derive(Debug, Serialize, Clone)]
pub struct PendingJourney {
    legs: Vec<Leg>,
    refresh_token: String,
    departure: DateTime<FixedOffset>,
    duration: Duration,
    prices: HashMap<DateTime<FixedOffset>, f32>,
    //summary: JourneySummary,
    // Leave Price, origin, destination out bc. they will be in the TripRecords
}

#[derive(Debug, Deserialize)]
pub struct Journey {
    id: ObjectId,
    legs: Vec<Leg>,
    refresh_token: String,
    departure: DateTime<FixedOffset>,
    duration: Duration,
}

/// A summary of the most important aspects of a journey
#[derive(Serialize, Deserialize, Clone)]
pub struct JourneySummary {
    refresh_token: String,
    departure: DateTime<FixedOffset>,
    duration: Duration,
    prices: HashMap<DateTime<FixedOffset>, f32>,
}

impl From<Vec<Journey>> for Journeys {
    fn from(journeys: Vec<Journey>) -> Self {
        Self(journeys)
    }
}

impl Journeys {
    pub fn new(journeys: Vec<Journey>) -> Self {
        Self(journeys)
    }
}

impl DocumentCollection for Journeys {
    type Document = Journey;
    fn add(&mut self, document: Self::Document) -> &mut Self::Document {
        self.0.push(document);
        self.0
            .last_mut()
            .expect("At least one value is in collection")
    }
}

impl MongoDocument for Journey {
    const COLLECTION: &'static str = "journey";
    fn id(&self) -> &ObjectId {
        &self.id
    }
}

impl InsertPendingDocument for PendingJourney {
    const COLLECTION: &'static str = "journeys";

    type Persisted = Journey;

    fn with_id(self, id: ObjectId) -> Self::Persisted {
        Self::Persisted {
            id,
            legs: self.legs,
            refresh_token: self.refresh_token,
            departure: self.departure,
            duration: self.duration,
        }
    }
}

impl PendingJourney {
    pub fn new(
        refresh_token: String,
        departure: DateTime<FixedOffset>,
        duration: Duration,
        prices: HashMap<DateTime<FixedOffset>, f32>,
        legs: Vec<Leg>,
    ) -> Self {
        Self {
            refresh_token,
            departure,
            duration,
            prices,
            legs,
        }
    }
}

/// Contains the same parameters as the vendo struct, but flattens the JSON structure
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Leg {
    origin: u32,
    destination: u32,
    departure: DateTime<FixedOffset>,
    arrival: DateTime<FixedOffset>,
    line: Option<String>,
}

impl Leg {
    pub fn new(
        origin: u32,
        destination: u32,
        departure: DateTime<FixedOffset>,
        arrival: DateTime<FixedOffset>,
        line: Option<String>,
    ) -> Self {
        Self {
            origin,
            destination,
            departure,
            arrival,
            line,
        }
    }
}

impl From<PendingJourney> for JourneySummary {
    fn from(pending: PendingJourney) -> Self {
        Self {
            refresh_token: pending.refresh_token,
            departure: pending.departure,
            duration: pending.duration,
            prices: pending.prices,
        }
    }
}
