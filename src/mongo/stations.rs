//! Match stations in MongoDB to trip stations

/// Stations come from a MongoDB collection, so their implementation should be located under mongo/
/// It is questionable, if the fuzzy matching should be here though, since its rather algorithmic and not IO
use crate::errors::ConnectionError;
use futures::stream::{StreamExt, TryStreamExt};

use fuzzy_match::fuzzy_match;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use unidecode::unidecode;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// Contains all long distaince train stations from Wikidata
pub struct Stations(pub(super) HashSet<Station>);

/// Dereference into [HashSet<Station>]
impl Deref for Stations {
    type Target = HashSet<Station>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<Vec<Station>> for Stations {
    fn from(value: Vec<Station>) -> Self {
        Self(HashSet::from_iter(value))
    }
}

impl Stations {
    pub fn try_get(&self, name: &str) -> Result<&Station, ConnectionError> {
        let deunicode = unidecode(name);
        match fuzzy_match(
            &deunicode,
            self.iter()
                .map(|station| (station.name.as_str(), station.ibnr)),
        ) {
            Some(ibnr) => {
                let station = self.iter().find(|station| station.ibnr == ibnr).unwrap();
                println!(
                    "Fuzzy matching successfull: match `{name:}` with `{}`",
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

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A train station from Wikidata
pub struct Station {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(rename = "Name", deserialize_with = "deserialize_unidecoded_string")]
    pub(crate) name: String,
    #[serde(rename = "IBNR")]
    ibnr: u32,
}

/// Deserialize and unidecode the [Station] name
fn deserialize_unidecoded_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(unidecode(&s))
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
    use super::super::client::MongoClient;
    use super::*;
    use crate::config::MONGO_CONNECTION_STRING;
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
        let mongo: MongoClient = MongoClient::try_connect(MONGO_CONNECTION_STRING).await?;
        let stations = Stations::from(mongo.load::<Station>().await?);
        let station = stations.try_get("Berlin Central Station")?;
        println!("{station:?}");
        Ok(())
    }

    #[test]
    fn duplicate_station_matching() -> Result<(), ConnectionError> {
        let a = "Berlin Central Station";
        let b = vec![("Berlin Central Station", 0), ("Berlin Central Station", 1)];
        match fuzzy_match(a, b) {
            Some(_) => {
                println!("found match");
                Ok(())
            }
            None => Err(ConnectionError::InvalidStation {
                name: a.to_string(),
            }),
        }
    }

    #[tokio::test]
    async fn unicode_station_name() -> Result<(), ConnectionError> {
        let mongo: MongoClient = MongoClient::try_connect(MONGO_CONNECTION_STRING).await?;
        let stations = Stations::from(mongo.load::<Station>().await?);
        let mut flag = false;
        for station in stations.iter() {
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
