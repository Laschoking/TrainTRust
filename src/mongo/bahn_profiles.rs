//! Profiles for Deutsche Bahn to collect parameters for Http GET request

//use crate::journey::Journey;
use super::client::{DocumentCollection, InsertPendingDocument, MongoDocument};
use crate::errors::ConnectionError;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Information for an API request to the Vendo endpoint
#[derive(Serialize)]
pub struct PendingBahnProfile {
    name: String,
    age: u8,
    tickets: bool,
    results: u8,
    first_class: bool,
    loyalty_card: LoyaltyCard,
    endpoint: String,
}

#[derive(Deserialize, Debug)]
pub struct BahnProfile {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    age: u8,
    tickets: bool,
    results: u8,
    first_class: bool,
    loyalty_card: LoyaltyCard,
    endpoint: String,
}

#[derive(Debug)]
pub struct BahnProfiles(Vec<BahnProfile>);

impl From<Vec<BahnProfile>> for BahnProfiles {
    fn from(value: Vec<BahnProfile>) -> Self {
        Self(value)
    }
}

impl DocumentCollection for BahnProfiles {
    type Document = BahnProfile;
    fn add(&mut self, document: Self::Document) -> &mut Self::Document {
        self.0.push(document);
        self.0
            .last_mut()
            .expect("At least one value is in collection")
    }
}

impl MongoDocument for BahnProfile {
    const COLLECTION: &'static str = "bahn_profiles";
    fn id(&self) -> &ObjectId {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Reduction cards supported by Deutsche Bahn
pub enum LoyaltyCard {
    None,
    C1BC25,
    C2BC25,
    C1BC50,
    C2BC50,
    C1BC100,
    C2BC100,
    Vorteilscard,
    HalbtaxaboRailplus,
    Halbtaxabo,
    VoordeelurenaboRailplus,
    Voordeelurenabo,
    Shcard,
    Generalabonnemen1st,
    Generalabonnement2nd,
    Generalabonnement,
    Nl40,
    AtKlimaticket,
}

impl InsertPendingDocument for PendingBahnProfile {
    const COLLECTION: &'static str = "bahn_profiles";
    type Persisted = BahnProfile;

    fn with_id(self, id: ObjectId) -> Self::Persisted {
        Self::Persisted {
            id,
            name: self.name,
            age: self.age,
            tickets: self.tickets,
            results: self.results,
            first_class: self.first_class,
            loyalty_card: self.loyalty_card,
            endpoint: self.endpoint,
        }
    }
}

impl PendingBahnProfile {
    pub fn new_with_options(
        name: String,
        age: Option<u8>,
        tickets: Option<bool>,
        results: Option<u8>,
        first_class: Option<bool>,
        loyalty_card: Option<&str>,
        endpoint: Option<String>,
    ) -> Result<PendingBahnProfile, ConnectionError> {
        let loyalty_card = LoyaltyCard::from_str(loyalty_card.unwrap_or("None"))?;
        Ok(Self {
            name,
            age: age.unwrap_or(30),
            tickets: tickets.unwrap_or(true),
            results: results.unwrap_or(10),
            first_class: first_class.unwrap_or(false),
            loyalty_card,
            endpoint: endpoint.unwrap_or(String::from("dbnav")),
        })
    }
}

impl BahnProfile {
    pub fn id(&self) -> &ObjectId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn as_hashmap(&self) -> HashMap<String, String> {
        HashMap::from([
            // Currently both fiels are removed because they are not used for the API request
            //(
            //    String::from("id"),
            //    self.id.map(|id| id.to_string()).unwrap_or_default(),
            //),
            //(String::from("name"), self.name.to_string()),
            (String::from("age"), self.age.to_string()),
            (String::from("tickets"), self.tickets.to_string()),
            (String::from("results"), self.results.to_string()),
            (String::from("firstClass"), self.first_class.to_string()),
            (String::from("loyaltyCard"), self.loyalty_card.to_string()),
            (String::from("profile"), self.endpoint.to_string()),
        ])
    }
}

impl BahnProfiles {
    /// Return a [BahnProfile] by its unique name
    pub fn find(&self, name: String) -> Option<&BahnProfile> {
        self.0.iter().find(|profile| profile.name == name)
    }
}

impl FromStr for LoyaltyCard {
    type Err = ConnectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "bahncard-1st-2" => Ok(Self::C1BC25),
            "bahncard-2nd-25" => Ok(Self::C2BC25),
            "bahncard-1st-50" => Ok(Self::C1BC50),
            "bahncard-2nd-50" => Ok(Self::C2BC50),
            "bahncard-1st-10" => Ok(Self::C1BC100),
            "bahncard-2nd-100" => Ok(Self::C2BC100),
            "vorteilscard" => Ok(Self::Vorteilscard),
            "halbtaxabo-railplus" => Ok(Self::HalbtaxaboRailplus),
            "halbtaxabo" => Ok(Self::Halbtaxabo),
            "voordeelurenabo-railplus" => Ok(Self::VoordeelurenaboRailplus),
            "voordeelurenabo" => Ok(Self::Voordeelurenabo),
            "shcard" => Ok(Self::Shcard),
            "generalabonnement-1st" => Ok(Self::Generalabonnemen1st),
            "generalabonnement-2nd" => Ok(Self::Generalabonnement2nd),
            "generalabonnement" => Ok(Self::Generalabonnement),
            "nl-40" => Ok(Self::Nl40),
            "at-klimaticket" => Ok(Self::AtKlimaticket),
            _ => {
                let values = [
                    "None",
                    "bahncard-1st-2",
                    "bahncard-2nd-25",
                    "bahncard-1st-50",
                    "bahncard-2nd-50",
                    "bahncard-1st-10",
                    "bahncard-2nd-100",
                    "vorteilscard",
                    "halbtaxabo-railplus",
                    "halbtaxabo",
                    "voordeelurenabo-railplus",
                    "voordeelurenabo",
                    "shcard",
                    "generalabonnement-1st",
                    "generalabonnement-2nd",
                    "generalabonnement",
                    "nl-40",
                    "at-klimaticket",
                ];
                Err(ConnectionError::InvalidBahnCard(
                    s.to_string(),
                    values.join(","),
                ))
            }
        }
    }
}

impl Display for LoyaltyCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::None => "None",
            Self::C1BC25 => "bahncard-1st-2",
            Self::C2BC25 => "bahncard-2nd-25",
            Self::C1BC50 => "bahncard-1st-50",
            Self::C2BC50 => "bahncard-2nd-50",
            Self::C1BC100 => "bahncard-1st-10",
            Self::C2BC100 => "bahncard-2nd-100",
            Self::Vorteilscard => "vorteilscard",
            Self::HalbtaxaboRailplus => "halbtaxabo-railplus",
            Self::Halbtaxabo => "halbtaxabo",
            Self::VoordeelurenaboRailplus => "voordeelurenabo-railplus",
            Self::Voordeelurenabo => "voordeelurenabo",
            Self::Shcard => "shcard",
            Self::Generalabonnemen1st => "generalabonnement-1st",
            Self::Generalabonnement2nd => "generalabonnement-2nd",
            Self::Generalabonnement => "generalabonnement",
            Self::Nl40 => "nl-40",
            Self::AtKlimaticket => "at-klimaticket",
        };
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::MONGO_CONNECTION_STRING, mongo::client::MongoClient};

    #[tokio::test]
    async fn insert_user() -> Result<(), ConnectionError> {
        let client = MongoClient::try_connect(MONGO_CONNECTION_STRING).await?;
        let mut profiles = client.load::<BahnProfiles>().await?;
        let pending = PendingBahnProfile::new_with_options(
            String::from("Karla"),
            Some(27),
            Some(true),
            None,
            Some(false),
            Some("bahncard-2nd-25"),
            None,
        )?;
        let user = client.persist_and_add(&mut profiles, pending).await?;
        println!("inserted user {} with id {:?}", user.name(), user.id());
        // Remove user from DB at the end
        client.drop::<BahnProfile>(user.id()).await?;

        Ok(())
    }
}
