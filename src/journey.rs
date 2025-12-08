//! A trip between two locations with price growth.

use chrono::{DateTime, Local, TimeDelta};

/// One entire trip between two [Station] that may be direct or require change
pub struct Journey {
    origin: Station,
    destination: Station,
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
    origin: Station,
    destination: Station,
    departure: DateTime<Local>,
    arrival: DateTime<Local>,
    train_type: String,
}

/// A trainstation with its IBNR
pub struct Station {
    name: String,
    ibnr: u32,
}

/// Links a price value to a timestamp
pub struct Price {
    currency: String,
    price: f32,
    date: DateTime<Local>,
}
