use serde::{Deserialize, Serialize};

use super::{deutsche_bahn::BahnProfile, journeys::JourneySummary};
use chrono::{DateTime, FixedOffset};
use std::collections::HashSet;

pub struct Trips(Vec<Trip>);

#[derive(Serialize, Deserialize)]
pub struct Trip {
    origin: StationIbnr,
    destination: StationIbnr,
    date: DateTime<FixedOffset>,
    user_profile: BahnProfile,
    journey_sum: Vec<JourneySummary>,
}

#[derive(Serialize, Deserialize)]
pub struct StationIbnr(String);
