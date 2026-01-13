//! Represents the core journeys, that can be deserialized from MongoDB or Http GET requests

use chrono::Duration;
use chrono::format::Fixed;
use chrono::{DateTime, FixedOffset, Local, TimeDelta};
use serde::{Deserialize, Serialize};

use crate::vendo::journeys::{JsonJourney, JsonLeg, JsonRequest};
use std::collections::{HashMap, HashSet};

///
///
#[derive(Debug)]
pub struct Journeys(Vec<Journey>);

impl Journeys {
    pub fn new(journeys: Vec<Journey>) -> Self {
        Self(journeys)
    }
}

/// This will serve as Serialization & Deserialization for MongoDB trips
#[derive(Debug, Serialize, Deserialize)]
pub struct Journey {
    legs: Vec<Leg>,
    refresh_token: String,
    departure: DateTime<FixedOffset>,
    duration: Duration,
    prices: HashMap<DateTime<FixedOffset>, f32>,
    //summary: JourneySummary,
    // Leave Price, origin, destination out bc. they will be in the TripRecords
}

impl Journey {
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Leg {
    // TODO: evaluate if it is better to introduce a new() function and reduce visibility
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

/// A summary of the most important aspects of a journey
#[derive(Serialize, Deserialize)]
pub struct JourneySummary {
    refresh_token: String,
    departure: DateTime<FixedOffset>,
    duration: Duration,
    prices: HashMap<DateTime<FixedOffset>, u32>,
}

impl JourneySummary {
    pub fn new(
        refresh_token: String,
        departure: DateTime<FixedOffset>,
        duration: Duration,
        prices: HashMap<DateTime<FixedOffset>, u32>,
    ) -> Self {
        Self {
            refresh_token,
            departure,
            duration,
            prices,
        }
    }
}
