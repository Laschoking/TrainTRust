//! Deserialize a GET response from the Vendo endpoint
use crate::domain::journeys::Leg;
use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{
    Client, Collection, Database,
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};
use serde::Deserialize;
use serde_aux::prelude::*;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
/// One entire trip between two [Station] that may be direct or require change
pub struct JourneyRequest {
    journeys: Vec<JsonJourney>,
}

#[derive(Deserialize, Debug)]
pub struct JsonJourney {
    price: Price,
    legs: Vec<JsonLeg>,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    tickets: Option<Vec<Ticket>>,
}

#[derive(Deserialize, Debug)]
/// One direct train hop between two [Station]s
pub(crate) struct JsonLeg {
    pub(crate) origin: Origin,
    pub(crate) destination: Destination,
    pub(crate) departure: DateTime<FixedOffset>,
    pub(crate) arrival: DateTime<FixedOffset>,
    pub(crate) line: Option<Line>,
}

impl From<JsonLeg> for Leg {
    fn from(leg: JsonLeg) -> Self {
        Self {
            origin: leg.origin.ibnr,
            destination: leg.destination.ibnr,
            departure: leg.departure,
            arrival: leg.arrival,
            line: leg.line.map(|line| line.name),
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Origin {
    #[serde(rename = "id", deserialize_with = "deserialize_number_from_string")]
    pub(crate) ibnr: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Destination {
    #[serde(rename = "id", deserialize_with = "deserialize_number_from_string")]
    pub(crate) ibnr: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Line {
    #[serde(rename = "id")]
    pub(crate) name: String,
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
        let _journey: JourneyRequest = serde_json::from_reader(reader)?;
        Ok(())
    }
}
