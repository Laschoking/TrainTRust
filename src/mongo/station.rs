//! Match stations in MongoDB to trip stations

use crate::errors::ConnectionError;
use futures::stream::{StreamExt, TryStreamExt};
use fuzzy_match::fuzzy_match;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use unidecode::unidecode;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A train station from Wikidata
pub struct Station {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "IBNR")]
    ibnr: u32,
}
impl Hash for Station {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Station {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Station {}

impl Station {
    pub fn ibnr(&self) -> u32 {
        self.ibnr
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;
    use crate::MongoClient;
    use crate::errors::ConnectionError;
    use fuzzy_match::fuzzy_match;

    #[test]
    fn remove_duplicates() {
        let station1 = Station {
            id: ObjectId::new(),
            name: "foo".to_string(),
            ibnr: 1,
        };
        let station2 = Station {
            id: ObjectId::new(),
            name: "foo".to_string(),
            ibnr: 2,
        };
        assert_eq!(station1, station2);
        let mut foo = HashSet::new();
        foo.insert(station1);
        foo.insert(station2);
        assert_eq!(foo.len(), 1);
    }

    #[tokio::test]
    async fn station_matching() -> Result<(), ConnectionError> {
        let uri = "mongodb://root:example@localhost:27017/?authSource=admin";
        let mongo = MongoClient::try_connect(uri).await?;
        let station_client = Stations::try_connect(mongo.database()).await?;
        let station = station_client.try_get("Berlin Central Station").await?;
        println!("{station:?}");
        Ok(())
    }

    #[test]
    fn bad_station() -> Result<(), ConnectionError> {
        let a = "Berlin Central Station";
        let b = vec![("Berlin Central Station", 0), ("Berlin Central Station", 1)];
        match fuzzy_match(a, b) {
            Some(t) => {
                println!("found match");
                Ok(())
            }
            None => Err(ConnectionError::InvalidStation {
                name: a.to_string(),
            }),
        }
    }

    #[tokio::test]
    async fn no_unicode_stations() -> Result<(), ConnectionError> {
        let uri = "mongodb://root:example@localhost:27017/?authSource=admin";
        let mongo = MongoClient::try_connect(uri).await?;
        let station_client = Stations::try_connect(mongo.database()).await?;
        let mut flag = false;
        for station in station_client.stations {
            if station.name.len() != station.name.chars().count() {
                flag = true;
                println!(
                    "station `{}` has len `{}`, but char count `{}`",
                    station.name,
                    station.name.len(),
                    station.name.chars().count()
                );
            }
        }
        if flag {
            Err(ConnectionError::InvalidCharacter)
        } else {
            Ok(())
        }
    }
}
// HINT: Hbf is Central Train Station
