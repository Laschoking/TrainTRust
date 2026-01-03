//! Client configuration to request and update trips and to sync with database.

use crate::{domain::controller::Controller, mongo::deutsche_bahn::BahnProfile};
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

    let controller = Controller::try_new().await?;
    //controller.load_data().await?;

    // main.rs accept user input, organize chrono runs

    // Connect to MongoDB
    // Load existing trips

    // Give trips to domain client for processing
    // domain: find trips to update & add new trip requests

    controller.update_trips().await?;
    // Vendo request updates & new trips
    // -> return to Domain the new trips & updates

    // find user or create new one
    let mut user = BahnProfile::new_with_options(
        String::from("Knut"),
        Some(27),
        Some(true),
        None,
        Some(false),
        Some("bahncard-2nd-25"),
        None,
    )?;

    let user = controller.add_user(&mut user).await?;

    let origin = "Frankfurt (Main) Hbf";
    let destination = "Berlin Central Station";
    let date = chrono::Local::now();

    //controller.new_trips(user, origin, destination, date).await?;

    // Analyze newest tendencies
    //controller.analyze_trends();

    // push updates to MongoDB
    //controller.update_db().await?;

    Ok(())
}

// params: HashMap<String, String> = profile.as_hashmap(date);
// result = vendo_socket.request(params.into_iter()).await?;
// journeys: JourneyRequest = serde_json::from_str(&result)?;
