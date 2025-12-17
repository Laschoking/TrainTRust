//! Defines structures to represent train trips accordingly

//use crate::journey::Journey;
use crate::stations::Station;
use chrono::{DateTime, Local, TimeDelta};
use mongodb::bson::oid::ObjectId;
use std::collections::{HashMap, HashSet};

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
impl LoyaltyCard {
    fn as_str(&self) -> String {
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
        res.to_string()
    }
}

/// Information for an API request to the Vendo endpoint
pub struct BahnProfile<'a> {
    origin: &'a Station,
    destination: &'a Station,
    mongo_id: Option<ObjectId>,
    age: u8,
    //computed_journeys: HashSet<Journey<'a>>,
    tickets: bool,
    results: u8,
    first_class: bool,
    loyalty_card: LoyaltyCard,
    endpoint: &'a str,
}

impl<'a> BahnProfile<'a> {
    pub fn new_with_options(
        origin: &'a Station,
        destination: &'a Station,
        age: Option<u8>,
        tickets: Option<bool>,
        results: Option<u8>,
        first_class: Option<bool>,
        loyalty_card: Option<LoyaltyCard>,
        endpoint: Option<&'a str>,
    ) -> Self {
        Self {
            origin,
            destination,
            age: age.unwrap_or(30),
            tickets: tickets.unwrap_or(true),
            results: results.unwrap_or(10),
            first_class: first_class.unwrap_or(false),
            loyalty_card: loyalty_card.unwrap_or(LoyaltyCard::None),
            endpoint: endpoint.unwrap_or("dbnav"),
            mongo_id: None,
            //computed_journeys: HashSet::new(),
        }
    }

    pub fn request_parameter(&self, date: DateTime<Local>) -> HashMap<String, String> {
        HashMap::from([
            (String::from("from"), self.origin.ibnr().to_string()),
            (String::from("to"), self.destination.ibnr().to_string()),
            (String::from("departure"), date.to_string()),
            (String::from("tickets"), self.tickets.to_string()),
            (String::from("results"), self.results.to_string()),
            (String::from("firstClass"), self.first_class.to_string()),
            (String::from("loyaltyCard"), self.loyalty_card.as_str()),
            (String::from("age"), self.age.to_string()),
            (String::from("profile"), self.endpoint.to_string()),
        ])
    }
}
