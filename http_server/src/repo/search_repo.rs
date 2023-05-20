use chrono::Utc;
use rocket_db_pools::Connection;
use sqlx::FromRow;
use time::Time;
use uuid::Uuid;

use crate::models::criteria::Criteria;
use crate::{XpressDB, MIN_ELAPSED_TIME};

#[derive(FromRow)]
pub struct Taxi {
    pub id: Uuid,
    pub available_seats: i64,
    pub rest_time: Time,
}

pub struct SearchRepo {
    conn: Connection<XpressDB>,
}

impl SearchRepo {
    pub fn new(conn: Connection<XpressDB>) -> SearchRepo {
        SearchRepo { conn }
    }
}

static ACTIVE_TAXIS_QUERY: &'static str =
    "SELECT t.id, (t.number_of_seats - count(b.reserved_seats)) AS available_seats, (tp.departure_time - $3)::time(0) AS rest_time 
    FROM trips tp 
        INNER JOIN taxis t on t.id = tp.taxi_id 
        LEFT JOIN bookings b on b.journey_id = tp.id 
            WHERE tp.origin_id = $1 AND tp.destination_id = $2 AND (tp.departure_time - $3) > '$4 min'
    GROUP BY t.id, tp.departure_time";

impl SearchRepo {
    pub async fn list_taxis_by_criteria(
        &mut self,
        criteria: Criteria<'_>,
    ) -> sqlx::Result<Vec<Taxi>> {
        sqlx::query_as::<_, Taxi>(ACTIVE_TAXIS_QUERY)
            .bind(criteria.origin)
            .bind(criteria.destination)
            .bind(Utc::now())
            .bind(MIN_ELAPSED_TIME)
            .fetch_all(&mut *self.conn)
            .await
    }
}
