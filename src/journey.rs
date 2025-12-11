//! A trip between two locations with price growth.

use crate::errors::ConnectionError;
use chrono::{DateTime, Local, TimeDelta};
use fuzzy_match::fuzzy_match;
use mongodb::{
    Client, Collection, Database,
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use futures::stream::{StreamExt, TryStreamExt};

/// One entire trip between two [Station] that may be direct or require change
pub struct Journey {
    origin: Station,
    destination: Station,
    departure: DateTime<Local>,
    arrival: DateTime<Local>,
    travelling_time: TimeDelta,
    last_updated: DateTime<Local>,
    prices: Vec<Price>,
    legs: Vec<Leg>,
    refresh_token: String,
}

/// One direct train hop between two [Station]s
pub struct Leg {
    origin: Station,
    destination: Station,
    departure: DateTime<Local>,
    arrival: DateTime<Local>,
    train_type: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
/// A trainstation with its IBNR
pub struct Station {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "IBNR")]
    ibnr: u32,
}

impl Station {
    async fn try_fuzzy_match(name: &str, mongo_con: &Database) -> Result<Self, ConnectionError> {
        let mut cursor: mongodb::Cursor<Station> = mongo_con
            .collection("stations")
            .find(doc! {})
            .projection(doc! {"_id": 1, "Name": 1, "IBNR" :1})
            .await?;

        // TODO ensure ordering of _id and Name
        let mut stations: Vec<Station> = Vec::new();
        while cursor.advance().await? {
            let station = cursor.deserialize_current()?;
            stations.push(station);
        }
        match fuzzy_match(
            name,
            stations
                .iter()
                .map(|station| (station.name.as_str(), station.ibnr)),
        ) {
            Some(ibnr) => {
                let station = stations
                    .iter()
                    .find(|station| station.ibnr == ibnr)
                    .unwrap();
                println!(
                    "Fuzzy matching successfull: match `{name:?}` with `{}`",
                    station.name
                );

                Ok(station.clone())
            }
            None => Err(ConnectionError::InvalidStation {
                name: name.to_string(),
            }),
        }
    }
}

/// Links a price value to a timestamp
pub struct Price {
    currency: String,
    price: f32,
    date: DateTime<Local>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MongoClient;
    use crate::errors::ConnectionError;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn station_matching() -> Result<(), ConnectionError> {
        let uri = "mongodb://root:example@localhost:27017/?authSource=admin";
        let mongo = MongoClient::try_connect(uri).await?;
        let station = Station::try_fuzzy_match("Dresden-Neustadt", mongo.database()).await?;
        println!("{station:?}");
        Ok(())
    }
}
// HINT: Hbf is Central Train Station
