//! A trip between two locations with price growth.

use crate::errors::ConnectionError;
use crate::stations::Station;
use chrono::{DateTime, Local, TimeDelta};
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{
    Client, Collection, Database,
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

//#[derive(Serialize, Deserialize)]
/// One entire trip between two [Station] that may be direct or require change
pub struct Journey<'a> {
    origin: &'a Station,
    destination: &'a Station,
    departure: DateTime<Local>,
    arrival: DateTime<Local>,
    travelling_time: TimeDelta,
    last_updated: DateTime<Local>,
    prices: Vec<Price>,
    legs: Vec<Leg>,
    refresh_token: String,
}

/// One direct train hop between two [Station]s
pub struct Leg {
    // TODO: it would be convenient to make origin and destination Stations
    // But this would mean that we make a lot of look-ups, and if the IBNR is not found
    // we will run into errors quickly
    // Maybe the Server response contains the IBNR?
    // Also it would be nice if Stations can stay immutable
    origin: String,
    destination: String,
    departure: DateTime<Local>,
    arrival: DateTime<Local>,
    train_type: String,
}

/// Links a price value to a timestamp
pub struct Price {
    currency: String,
    price: f32,
    date: DateTime<Local>,
}
