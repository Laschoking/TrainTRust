//! Match stations in MongoDB to trip stations

use crate::errors::ConnectionError;
use fuzzy_match::fuzzy_match;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};

use futures::stream::{StreamExt, TryStreamExt};

#[derive(Debug, serde::Deserialize, Clone)]
/// A train station from Wikidata
pub struct Station {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "IBNR")]
    ibnr: u32,
}
// Here we could also use macros no?
impl Station {
    pub fn ibnr(&self) -> u32 {
        self.ibnr
    }
}

pub struct Stations {
    stations: Vec<Station>,
}

impl Stations {
    pub async fn try_connect(mongo: &Database) -> Result<Self, ConnectionError> {
        let mut cursor: mongodb::Cursor<Station> = mongo
            .collection("stations")
            .find(doc! {})
            .projection(doc! {"_id": 1, "Name": 1, "IBNR" :1})
            .await?;

        let mut stations: Vec<Station> = Vec::new();
        while cursor.advance().await? {
            let station = cursor.deserialize_current()?;
            stations.push(station);
        }
        Ok(Self { stations })
    }

    pub async fn try_get(&self, name: &str) -> Result<&Station, ConnectionError> {
        match fuzzy_match(
            name,
            self.stations
                .iter()
                .map(|station| (station.name.as_str(), station.ibnr)),
        ) {
            Some(ibnr) => {
                let station = self
                    .stations
                    .iter()
                    .find(|station| station.ibnr == ibnr)
                    .unwrap();
                println!(
                    "Fuzzy matching successfull: match `{name:?}` with `{}`",
                    station.name
                );

                Ok(station)
            }
            None => Err(ConnectionError::InvalidStation {
                name: name.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MongoClient;
    use crate::errors::ConnectionError;

    #[tokio::test]
    async fn station_matching() -> Result<(), ConnectionError> {
        let uri = "mongodb://root:example@localhost:27017/?authSource=admin";
        let mongo = MongoClient::try_connect(uri).await?;
        let station_client = Stations::try_connect(mongo.database()).await?;
        let station = station_client.try_get("Dresden-Neustadt").await?;
        println!("{station:?}");
        Ok(())
    }
}
// HINT: Hbf is Central Train Station
