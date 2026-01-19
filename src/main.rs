//! Client configuration to request and update trips and to sync with database.

use crate::{
    domain::controller::Controller,
    mongo::bahn_profiles::PendingBahnProfile,
};
use chrono::Local;

mod config;
mod domain;
mod errors;
mod mongo;
mod vendo;

#[tokio::main]
async fn main() -> Result<(), errors::ConnectionError> {
    // Main should do:: initiate clients
    // First: connect to MongoDB & update price information
    // Second: make new price requests for new journeys
    // Potentially take command line arguments (from, to, date)

    let mut controller = Controller::try_new().await?;
    //controller.load_data().await?;

    // main.rs accept user input, organize chrono runs

    // Connect to MongoDB
    // Load existing trips

    // Give trips to domain client for processing
    // domain: find trips to update & add new trip requests

    // TODO controller.update_trips().await?;
    // Vendo request updates & new trips
    // -> return to Domain the new trips & updates

    // find user or create new one
    let pending = PendingBahnProfile::new_with_options(
        String::from("Knut"),
        Some(27),
        Some(true),
        None,
        Some(false),
        Some("bahncard-2nd-25"),
        None,
    )?;

    let user_name = controller.insert_user(pending).await?;

    let origin = "Frankfurt (Main) Hbf";
    let destination = "Berlin Central Station";
    let date = Local::now().with_timezone(Local::now().offset());

    //
    controller
        .add_trip(&user_name, origin, destination, date)
        .await?;

    controller.update_trips().await?;

    // UPdates: only Journey Summary, the rest should be fine,

    // Analyze newest tendencies
    //controller.analyze_trends();

    // push updates to MongoDB
    //controller.update_db().await?;

    Ok(())
}
