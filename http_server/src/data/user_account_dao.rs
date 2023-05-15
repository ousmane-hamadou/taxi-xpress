use crate::{data, XpressDB};
use data::taxi_dao::Taxi;
use rocket_db_pools::Connection;
use sqlx::{Acquire, FromRow};
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub number: String,
    pub password: String,
}

pub struct UserDto {
    pub id: Uuid,
    pub password: String,
    pub taxi: Taxi,
}

pub async fn insert(conn: &mut Connection<XpressDB>, user: &UserDto) -> sqlx::Result<()> {
    let mut tx = conn.begin().await?;
    let taxi = &user.taxi;

    sqlx::query("INSERT INTO taxis(id, number, max_place, available_place, current_station, destination_station)\
                    VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&taxi.id)
        .bind(&taxi.number)
        .bind(taxi.max_place)
        .bind(taxi.available_place)
        .bind(taxi.current_station)
        .bind(taxi.destination_station)
        .execute(&mut *tx).await?;

    sqlx::query("INSERT INTO user_accounts(id, number, password) VALUES($1, $2, $3)")
        .bind(&user.id)
        .bind(&taxi.number)
        .bind(&user.password)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn from_number(
    conn: &mut Connection<XpressDB>,
    number: &str,
) -> sqlx::Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT id, password, number FROM user_accounts WHERE number = $1")
        .bind(number)
        .fetch_optional(&mut **conn)
        .await
}
