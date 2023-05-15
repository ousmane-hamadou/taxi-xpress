use crate::XpressDB;
use rocket_db_pools::Connection;
use sqlx::{FromRow, Row};
use uuid::Uuid;

#[derive(FromRow)]
pub struct Taxi {
    pub id: Uuid,
    pub number: String,
    pub max_place: i32,
    pub available_place: i32,
    pub current_station: Uuid,
    pub destination_station: Uuid,
}

pub struct Station {
    pub arrival: Uuid,
    pub departure: Uuid,
}

pub async fn from_id(conn: &mut Connection<XpressDB>, id: Uuid) -> sqlx::Result<Option<Taxi>> {
    sqlx::query_as::<_, Taxi>("SELECT * FROM taxis WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **conn)
        .await
}

pub async fn update_available_place(
    conn: &mut Connection<XpressDB>,
    id: Uuid,
    place: i32,
) -> sqlx::Result<u64> {
    Ok(
        sqlx::query("UPDATE taxi SET available_place = $1 WHERE id = $2")
            .bind(place)
            .bind(id)
            .execute(&mut **conn)
            .await?
            .rows_affected(),
    )
}

pub async fn update_station(
    conn: &mut Connection<XpressDB>,
    id: &Uuid,
    station: &Station,
) -> sqlx::Result<u64> {
    Ok(
        sqlx::query("UPDATE taxis SET current_station=$1,  destination_station = $2 WHERE = $3")
            .bind(&station.departure)
            .bind(&station.arrival)
            .bind(id)
            .execute(&mut **conn)
            .await?
            .rows_affected(),
    )
}

pub async fn from_station(
    conn: &mut Connection<XpressDB>,
    station: Station,
) -> sqlx::Result<Vec<Taxi>> {
    Ok(sqlx::query("SELECT id, number, max_place, available_place FROM taxis WHERE current_station = $1 AND destination_station = $2")
        .bind(&station.departure)
        .bind(&station.arrival)
        .map(move |row| {
            let id = row.try_get("id").unwrap();
            let number = row.try_get("number").unwrap();
            let available_place = row.try_get("available_place").unwrap();
            let max_place = row.try_get("max_place").unwrap();

            Taxi {
                id,
                number,
                available_place,
                max_place,
                current_station: station.departure,
                destination_station: station.arrival,
            }
        }).fetch_all(&mut **conn).await?)
}
