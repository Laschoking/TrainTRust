//! Deserialize a GET response from the Vendo endpoint
use crate::mongo::journeys::{Leg, PendingJourney};
use chrono::{DateTime, FixedOffset, Local};
use mongodb::bson::doc;
use serde::Deserialize;
use serde_aux::prelude::*;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct JsonRequest {
    journeys: Vec<JsonJourney>,
}

/// One entire trip between two [Station] that may be direct or require change
#[derive(Deserialize, Debug)]
pub struct JsonJourney {
    price: Price,
    legs: Vec<JsonLeg>,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    tickets: Option<Vec<Ticket>>,
}

/// One direct train hop between [Origin] and [Destination]
#[derive(Deserialize, Debug)]
pub(crate) struct JsonLeg {
    pub(crate) origin: Origin,
    pub(crate) destination: Destination,
    pub(crate) departure: DateTime<FixedOffset>,
    pub(crate) arrival: DateTime<FixedOffset>,
    pub(crate) line: Option<Line>,
}

impl From<JsonRequest> for Vec<PendingJourney> {
    fn from(data: JsonRequest) -> Self {
        data.journeys
            .into_iter()
            .map(|journey| PendingJourney::from(journey))
            .collect()
    }
}

impl From<JsonJourney> for PendingJourney {
    fn from(journey: JsonJourney) -> Self {
        assert!(!journey.legs.is_empty());
        let departure = journey.legs[0].departure;
        let arrival = journey
            .legs
            .last()
            .expect("Journey consists of at least one leg")
            .arrival;
        assert!(departure < arrival);
        let duration = arrival - departure;
        let prices = HashMap::from([(Local::now().fixed_offset(), journey.price.amount)]);
        PendingJourney::new(
            journey.refresh_token,
            departure,
            duration,
            prices,
            journey.legs.into_iter().map(Into::into).collect(),
        )
        // TODO implement JourneySummary either as separate (Journey, JourneySummary) or as field of Journey, but then it has to be separated in Mongo again
        // Maybe HashMap<Journey, JourneySummary> ot sht
    }
}

//impl From<JsonJourney> for JourneySummary {
//    fn from(journey: JsonJourney) -> Self {
//        assert!(!journey.legs.is_empty());
//        let departure = journey.legs[0].departure;
//        let arrival = journey
//            .legs
//            .last()
//            .expect("Journey consists of at least one leg")
//            .arrival;
//        assert!(departure < arrival);
//        let duration = arrival - departure;
//        let prices = HashMap::from([(Local::now().fixed_offset(), journey.price.amount)]);
//        JourneySummary::new(journey.refresh_token, departure, duration, prices)
//    }
//}

impl From<JsonLeg> for Leg {
    fn from(leg: JsonLeg) -> Self {
        Self::new(
            leg.origin.ibnr,
            leg.destination.ibnr,
            leg.departure,
            leg.arrival,
            leg.line.map(|line| line.name),
        )
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
        let _journey: JsonRequest = serde_json::from_reader(reader)?;
        Ok(())
    }
}
