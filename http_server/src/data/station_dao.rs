use crate::XpressDB;
use rocket_db_pools::Connection;
use sqlx::{FromRow, Row};
use uuid::Uuid;

#[derive(FromRow)]
pub struct Station {
    pub id: Uuid,
    pub name: String,
}

pub async fn from_id(conn: &mut Connection<XpressDB>, id: Uuid) -> sqlx::Result<Option<Station>> {
    sqlx::query("SELECT name FROM stations WHERE id = $1")
        .bind(&id)
        .map(move |row| {
            let name = row.try_get("name").unwrap();
            Station { id, name }
        })
        .fetch_optional(&mut **conn)
        .await
}

pub async fn list_all(conn: &mut Connection<XpressDB>) -> sqlx::Result<Vec<Station>> {
    sqlx::query_as::<_, Station>("SELECT id, name FROM stations")
        .fetch_all(&mut **conn)
        .await
}
