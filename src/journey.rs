//! A trip between two locations with price growth.

use crate::errors::ConnectionError;
use crate::stations::Station;
use chrono::{DateTime, Local, TimeDelta, Utc};
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{
    Client, Collection, Database,
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use thiserror::Error;

//pub struct Journey <'a> {
//    origin: &'a Station,
//    destination: &'a Station,
//    departure: DateTime<Local>,
//    arrival: DateTime<Local>,
//    Legs<
//    travelling_time: TimeDelta,
//    last_updated: DateTime<Local>,
//}
//
//impl From<JourneyData> for Journey {
//    fn from(value: JourneyData) -> Self {}
//}

#[derive(Deserialize, Debug)]
/// One entire trip between two [Station] that may be direct or require change
pub struct JourneyData {
    journeys: Vec<RawJourney>,
}

#[derive(Deserialize, Debug)]
pub struct RawJourney {
    price: Price,
    legs: Vec<Leg>,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    tickets: Option<Vec<Ticket>>,
}

#[derive(Deserialize, Debug)]
/// One direct train hop between two [Station]s
pub struct Leg {
    // TODO: it would be convenient to make origin and destination Stations
    // But this would mean that we make a lot of look-ups, and if the IBNR is not found
    // we will run into errors quickly
    // Maybe the Server response contains the IBNR?
    // Also it would be nice if Stations can stay immutable
    origin: Origin,
    destination: Destination,
    departure: DateTime<Utc>,
    arrival: DateTime<Utc>,
    line: Option<Line>,
}

#[derive(Deserialize, Debug)]
pub struct Origin {
    #[serde(rename = "id", deserialize_with = "deserialize_number_from_string")]
    ibnr: u32,
}

#[derive(Deserialize, Debug)]
pub struct Destination {
    #[serde(rename = "id", deserialize_with = "deserialize_number_from_string")]
    ibnr: u32,
}

#[derive(Deserialize, Debug)]
pub struct Line {
    #[serde(rename = "id")]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Price {
    currency: String,
    amount: f32,
}

#[derive(Deserialize, Debug)]
pub struct Ticket {
    name: String,
    #[serde(rename = "priceObj")]
    price_obj: PriceObj,
}

#[derive(Deserialize, Debug)]
pub struct PriceObj {
    amount: u32,
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn parse_json() -> Result<(), serde_json::Error> {
        let file = File::open("data/test_journey.json").unwrap();
        let reader = BufReader::new(file);
        let journey: JourneyData = serde_json::from_reader(reader)?;
        Ok(())
    }
}
