//! Client configuration to request and update trips and to sync with database.

use crate::mongo::mongo_client;

mod deutsche_bahn;
mod errors;
mod journey;
mod mongo;
mod vendo_socket;

//use journey;

fn main() -> Result<(), errors::ConnectionError> {
    let uri = "mongodb://root:example@localhost:27017/?authSource=admin";

    let database = mongo_client(uri)?;

    Ok(())
}
