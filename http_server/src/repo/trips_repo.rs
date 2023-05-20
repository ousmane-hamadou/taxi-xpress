use crate::XpressDB;
use chrono::{DateTime, Utc};
use rocket_db_pools::Connection;
use uuid::Uuid;

const INSERT_JOURNEY: &'static str = "INSERT INTO trips (id, taxi_id, origin_id, destination_id, departure_time) VALUES ($1, $2, $3, $4, $5)";

pub struct TripsRepo {
    conn: Connection<XpressDB>,
}

pub struct Journey {
    pub id: Uuid,
    pub taxi_id: Uuid,
    pub origin_id: Uuid,
    pub destination_id: Uuid,
    pub departure_schedule: DateTime<Utc>,
}

impl TripsRepo {
    pub fn new(conn: Connection<XpressDB>) -> Self {
        TripsRepo { conn }
    }

    pub async fn start_journey(&mut self, journey: &Journey) -> sqlx::Result<u64> {
        Ok(sqlx::query(INSERT_JOURNEY)
            .bind(&journey.id)
            .bind(&journey.taxi_id)
            .bind(&journey.origin_id)
            .bind(&journey.destination_id)
            .bind(&journey.departure_schedule)
            .execute(&mut *self.conn)
            .await?
            .rows_affected())
    }
}
