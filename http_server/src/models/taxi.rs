use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Taxi {
    id: Uuid,
    number: String,
    number_of_seats: u8,
}
