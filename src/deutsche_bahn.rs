//! Defines structures to represent train trips accordingly

use crate::journey::{Journey, Station};
use std::collections::HashSet;

/// Reduction cards supported by Deutsche Bahn
pub enum LoyalityCard {
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
impl LoyalityCard {
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
pub struct BahnProfile {
    origin: Station,
    destination: Station,
    mongo_id: u32,
    age: u8,
    computed_journeys: HashSet<Journey>,
    tickets: bool,
    results: u8,
    first_class: bool,
    loyalty_card: LoyalityCard,
    endpoint: String,
}

impl BahnProfile {}
