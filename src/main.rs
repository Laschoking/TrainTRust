//! Client configuration to request and update trips and to sync with database.

use crate::{mongo::MongoClient, vendo_socket::VendoSocket};
use std::collections::HashMap;

mod deutsche_bahn;
mod errors;
mod journey;
mod mongo;
mod vendo_socket;

//use journey;

#[tokio::main]
async fn main() -> Result<(), errors::ConnectionError> {
    let mongo_con = "mongodb://root:example@localhost:27017/?authSource=admin";
    let mongo = MongoClient::try_connect(mongo_con).await?;

    let vendo_uri = "https://v6.db.transport.rest/journeys";
    let vendo_socket = VendoSocket::try_from(vendo_uri)?; //.map_err(|err| err.into())?;
    let params = HashMap::from([("X", "Y")]);
    vendo_socket.request(params.into_iter())?;

    Ok(())
}

// TODO: implement Serde serialize and deserialize for Journey & Db-profile
