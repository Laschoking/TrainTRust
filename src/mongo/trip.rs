use serde::{Deserialize, Serialize};

use super::{deutsche_bahn::BahnProfile, journeys::JourneySummary, stations::Station};
use chrono::{DateTime, FixedOffset};
use std::collections::HashSet;

pub struct Trips(Vec<Trip>);

impl Trips {
    /// Finds an existing trip for specified parameters
    pub fn find(
        &self,
        user_name: String,
        origin: &StationIbnr,
        destination: &StationIbnr,
        date: DateTime<FixedOffset>,
    ) -> Option<&Trip> {
        self.0.iter().find(|trip| {
            trip.user == user_name
                && trip.origin == *origin
                && trip.destination == *destination
                && trip.date == date
        })
    }

    /// Add a new [Trip] to [Trips]
    pub fn add(&mut self, trip: Trip) {
        self.0.push(trip);
    }
}

/// Contains a compressed view on travelling connections based on the parameters
/// origin, destination, date, user_profile
#[derive(Serialize, Deserialize)]
pub struct Trip {
    origin: StationIbnr,
    destination: StationIbnr,
    date: DateTime<FixedOffset>,
    // The unique identifier name of a [BahnProfile]
    user: String,
    // TODO: For Deserialization this could be empty, or can we expect at least one journey per trip
    journey_sum: Vec<JourneySummary>,
}

impl Trip {
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

    /// Extract parameters for the API request
    pub fn HTTP_params(&self) -> HashMap<String, String> {
        HashMap::from([
            (String::from("from"), self.origin),
            (String::from("to"), self.destination),
            (String::from("date"), self.date),
        ])
    }

    /// Update a the [JourneySummary] of a [Trip]
    pub fn update(&mut self, trip: &Trip) {
        // TODO Update the JourneySUmmary here:
        todo!()
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct StationIbnr(u32);

impl From<&Station> for StationIbnr {
    fn from(station: &Station) -> Self {
        Self(station.ibnr())
    }
}
