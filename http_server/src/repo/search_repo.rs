use chrono::Utc;
use rocket_db_pools::Connection;
use sqlx::{FromRow, Row};
use time::Time;
use uuid::Uuid;

use crate::models::criteria::Criteria;
use crate::{XpressDB};

#[derive(FromRow)]
pub struct Taxi {
    pub number: String,
    pub available_seats: i32,
    pub rest_time: Time,
    pub model: String,
    pub journey_id: Uuid,
}

#[derive(FromRow)]
pub struct SelectedTaxi {
    pub number: String,
    pub number_of_seats: i32,
    pub model: String,
}

#[derive(FromRow)]
pub struct JourneySeats {
    pub total: i32,
    pub reserved: Option<i32>,
}

pub struct SearchRepo {
    conn: Connection<XpressDB>,
}

impl SearchRepo {
    pub fn new(conn: Connection<XpressDB>) -> SearchRepo {
        SearchRepo { conn }
    }
}

const ACTIVE_TAXIS_QUERY: &'static str =
    "SELECT t.number, tp.id, t.model, t.number_of_seats, sum(b.reserved_seats) AS reserved, (tp.departure_time - $3)::time(0) AS rest_time
    FROM trips tp 
        INNER JOIN taxis t on t.id = tp.taxi_id 
        LEFT JOIN bookings b on b.journey_id = tp.id 
            WHERE tp.origin_id = $1 AND tp.destination_id = $2
    GROUP BY t.number, t.model, t.number_of_seats, tp.departure_time, tp.id";

const SEATS_FOR_JOURNEY: &'static str =
    "SELECT t.number_of_seats as total, sum(b.reserved_seats) as reserved FROM trips tp
        INNER JOIN taxis t on t.id = tp.taxi_id
        LEFT JOIN bookings b on b.journey_id = tp.id
            WHERE tp.id = $1
    GROUP BY t.number_of_seats";

const FETCH_TAXIS: &'static str =
    "SELECT t.model, t.number, (tp.departure_time - $3)::time(0) AS departure_time FROM trips tp
            INNER JOIN taxis t on t.id = tp.taxi_id
                WHERE tp.id = $1
    GROUP BY t.id, tp.departure_time";


impl SearchRepo {
    pub async fn list_taxis_by_criteria(
        &mut self,
        criteria: Criteria<'_>,
    ) -> sqlx::Result<Vec<Taxi>> {
        sqlx::query(ACTIVE_TAXIS_QUERY)
            .bind(criteria.origin)
            .bind(criteria.destination)
            .bind(Utc::now())
            .map(|row| {
                let number: String = row.try_get("number").unwrap();
                let reserved_seats: Option<i32> = row.try_get("reserved").unwrap();
                let number_of_seats: i32 = row.try_get("number_of_seats").unwrap();
                let model: String = row.try_get("model").unwrap();
                let rest_time: Time = row.try_get("rest_time").unwrap();
                let journey_id: Uuid = row.try_get("id").unwrap();
                let available_seats = reserved_seats.map_or_else(|| number_of_seats, |n| number_of_seats - reserved_seats.unwrap_or(0));
                Taxi { number, model, available_seats,rest_time, journey_id }
            })
            .fetch_all(&mut *self.conn)
            .await
    }

    pub async fn get_journey_seats(&mut self, journey_id: &Uuid) -> sqlx::Result<Option<JourneySeats>> {
        sqlx::query_as::<_, JourneySeats>(SEATS_FOR_JOURNEY)
            .bind(journey_id)
            .fetch_optional(&mut *self.conn).await
    }

    pub async fn get_selected_taxi(&mut self, id: &Uuid) -> sqlx::Result<Taxi> {
        todo!()
    }
}
