use crate::{
    config::{MONGO_CONNECTION_STRING, VENDO_URI},
    errors::ConnectionError,
    mongo::{
        bahn_profiles::{BahnProfiles, PendingBahnProfile},
        client::MongoClient,
        journeys::{JourneySummary, Journeys, PendingJourney},
        stations::Stations,
        trips::{PendingTrip, Trips},
    },
    vendo::{client::VendoSocket, journeys::JsonRequest},
};
use chrono::{DateTime, FixedOffset};

/// Orchestrates the API client and the data flow in/out of MongoDB database
pub struct Controller {
    // TODO: become selective in loading Data
    /// Handles document manipulation for MongoDB
    db_client: MongoClient,
    /// Handles connection and requests to the vendo API
    vendo_socket: VendoSocket,
    /// All trips stored in MongoDB
    trips: Trips,
    /// Unique stations in MongoDB (retrieved from Wikidata)
    stations: Stations,
    /// All [BahnProfile]s that are stored in MongoDB
    profiles: BahnProfiles,
    /// Contains collection of [Journey] with travel information
    journeys: Journeys,
}

impl Controller {
    /// Initiate connection to [VendoSocket] and load trip data from [MongoClient]
    pub async fn try_new() -> Result<Self, ConnectionError> {
        let db_client = MongoClient::try_connect(MONGO_CONNECTION_STRING).await?;
        let vendo_socket = VendoSocket::try_from(VENDO_URI)?;
        let stations = db_client.load::<Stations>().await?;
        // TODO Potentially at some point we dont want to load everything anymore, but only some filtered data
        let trips = db_client.load::<Trips>().await?;
        let profiles = db_client.load::<BahnProfiles>().await?;
        let journeys = db_client.load::<Journeys>().await?;

        Ok(Self {
            db_client,
            vendo_socket,
            trips,
            stations,
            profiles,
            journeys,
        })
    }

    /// maybe an Insert if doc not in DB?
    /// Insert new profile with [BahnProfile] to MongoDB
    pub async fn insert_user(
        &mut self,
        pending: PendingBahnProfile,
    ) -> Result<String, ConnectionError> {
        // TODO: verify if profile is maybe existing already in the in-memory collection
        let user = self
            .db_client
            .persist_and_add(&mut self.profiles, pending)
            .await?;
        Ok(user.name().clone().to_owned())
    }

    /// Adds a new [Trip] to [Trips]
    pub async fn add_trip(
        &mut self,
        user_name: &String,
        origin: &str,
        destination: &str,
        date: DateTime<FixedOffset>,
    ) -> Result<(), ConnectionError> {
        let origin = self.stations.try_get(origin)?.into();
        let destination = self.stations.try_get(destination)?.into();

        if self
            .trips
            .find(user_name, &origin, &destination, date)
            .is_none()
        {
            let pending = PendingTrip::new(user_name.clone(), origin, destination, date);
            let _ = self
                .db_client
                .persist_and_add(&mut self.trips, pending)
                .await?;
        };
        Ok(())
    }

    pub async fn update_trips(&mut self) -> Result<(), ConnectionError> {
        for trip in self.trips.iter_mut() {
            let Some(profile) = self.profiles.find(trip.user()) else {
                panic!(
                    "User profile with name `{}` does not exist in profiles {:?}",
                    trip.user(),
                    self.profiles
                );
            };
            let mut params = profile.as_hashmap();
            params.extend(trip.http_params());
            let json = self.vendo_socket.request(params).await?;
            let data: JsonRequest = serde_json::from_str(json.as_str())?;
            let pendings: Vec<PendingJourney> = data.into();
            for pending in pendings {
                let summary: JourneySummary = pending.clone().into();
                self.db_client
                    .persist_and_add(&mut self.journeys, pending)
                    .await?;
                self.db_client.push_trip_summary(trip, summary).await?;
            }
        }
        Ok(())
    }
}
