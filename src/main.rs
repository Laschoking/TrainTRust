//! Client configuration to request and update trips and to sync with database.

use crate::{
    deutsche_bahn::{BahnProfile, LoyaltyCard},
    journey::Journey,
    mongo::MongoClient,
    stations::Stations,
    vendo_socket::VendoSocket,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod deutsche_bahn;
mod errors;
mod journey;
mod mongo;
mod stations;
mod vendo_socket;

#[tokio::main]
async fn main() -> Result<(), errors::ConnectionError> {
    let mongo_con = "mongodb://root:example@localhost:27017/?authSource=admin";
    let mongo = MongoClient::try_connect(mongo_con).await?;

    let vendo_uri = "https://v6.db.transport.rest/journeys";
    let vendo_socket = VendoSocket::try_from(vendo_uri)?;

    let stations = Stations::try_connect(mongo.database()).await?;

    let origin = stations.try_get("Frankfurt (Main) Hbf").await?;
    let destination = stations.try_get("Berlin Central Station").await?;

    let profile = BahnProfile::new_with_options(
        origin,
        destination,
        None,
        None,
        None,
        None,
        Some(LoyaltyCard::C2BC25),
        None,
    );

    //let dt: NaiveDateTime =
    //  NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap()
    //.with_ymd_and_hms(2025, 12, 17, 06, 0, 0)
    let date = chrono::Local::now();
    let params: HashMap<String, String> = profile.request_parameter(date);
    let result = vendo_socket.request(params.into_iter()).await?;

    // TODO Deserialization into Journey (besides Legs) or deserialization function
    //let journey: Journey = serde_json::from_str(&result)?;
    println!("{result}");

    Ok(())
}
// Station: Error or Optional?
