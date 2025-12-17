//! Represents the core journeys, that can be deserialized from MongoDB or Http GET requests

use chrono::Duration;
use chrono::{DateTime, Local, TimeDelta};

use crate::{
    mongo::journeys::JourneyDocument,
    vendo::journeys::{JourneyRequest, JsonJourney, JsonLeg},
};
use std::collections::{HashMap, HashSet};
///
pub struct Journeys {
    journeys: HashSet<Journey>,
}

impl From<JourneyRequest> for Journeys {
    fn from(journey_data: JourneyRequest) -> Self {}
}

//impl From<JourneyDocument> for Journeys {}

/// This will serve as Serialization & Deserialization for MongoDB trips
pub struct Journey {
    // Leave Price, origin, destination out bc. they will be in the TripRecords
}

impl From<JsonJourney> for Journey {
    fn from(value: JsonJourney) -> Self {}
}

/// Contains the same parameters as the vendo struct, but flattens the JSON structure
pub struct Leg {
    // TODO: evaluate if it is better to introduce a new() function and reduce visibility
    pub(crate) origin: u32,
    pub(crate) destination: u32,
    pub(crate) departure: DateTime<FixedOffset>,
    pub(crate) arrival: DateTime<FixedOffset>,
    pub(crate) line: Option<String>,
}

/// A summary of the most important aspects of a journey
pub struct JourneySummary {
    refresh_token: String,
    departure: DateTime<Local>,
    duration: Duration,
    prices: HashMap<DateTime<Local>, u32>, // or PriceObj?
}
