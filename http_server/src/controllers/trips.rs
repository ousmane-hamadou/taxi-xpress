use chrono::{DateTime, Utc};
use rocket::serde::json::Json;
use rocket::{post, routes, Route};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Error;
use crate::models::taxi_owner::TaxiOwner;
use crate::repo::trips_repo;
use crate::repo::trips_repo::TripsRepo;
use crate::utils::get_uuid_from_station_url;

#[derive(Deserialize)]
struct Journey<'r> {
    departure_time: DateTime<Utc>,
    origin: &'r str,
    destination: &'r str,
}

#[derive(Serialize)]
struct CreatedJourney {
    id: Uuid,
    origin_id: String,
    destination_id: String,
    departure_time: DateTime<Utc>,
}

impl Into<CreatedJourney> for trips_repo::Journey {
    fn into(self) -> CreatedJourney {
        CreatedJourney {
            id: self.id,
            origin_id: String::new(),
            destination_id: String::new(),
            departure_time: self.departure_schedule,
        }
    }
}

#[post("/trips", format = "json", data = "<journey>")]
async fn start_trip(
    journey: Json<Journey<'_>>,
    owner: TaxiOwner,
    mut repo: TripsRepo,
) -> Result<Json<CreatedJourney>, Error> {
    let trip = trips_repo::Journey {
        id: Uuid::new_v4(),
        origin_id: get_uuid_from_station_url(journey.origin),
        destination_id: get_uuid_from_station_url(journey.destination),
        departure_schedule: journey.departure_time,
        taxi_id: owner.taxi_id,
    };

    repo.start_journey(&trip).await?;

    let mut response: CreatedJourney = trip.into();
    response.origin_id = journey.origin.into();
    response.destination_id = journey.destination.into();

    Ok(Json(response))
}

pub fn routes() -> Vec<Route> {
    routes![start_trip]
}
