use crate::domain::deutsche_bahn::BahnProfile;

/// A document that will connect Journeys and a profile, for a certain trip
#[derive(serde::{Deserialize, Serialize})]
pub struct Trip<'a> {
    user_profile: &BahnProfile,
    origin: &'a Station,
    destination: &'a Station,
    date: DateTime<Local>,
}
