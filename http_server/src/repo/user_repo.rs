use rocket_db_pools::Connection;
use sqlx::FromRow;
use uuid::Uuid;

use crate::XpressDB;

const FETCH_USER_BY_TAXI_NUM: &'static str =
    "SELECT a.id, t.id AS taxi_id, a.full_name, a.password FROM user_accounts a \
        INNER JOIN taxis t ON t.number = a.id \
            WHERE a.id = $1";

pub struct UserRepo {
    conn: Connection<XpressDB>,
}

#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub full_name: String,
    pub taxi_id: Uuid,
    pub password: String,
}

impl UserRepo {
    pub fn new(conn: Connection<XpressDB>) -> Self {
        UserRepo { conn }
    }

    pub async fn fetch_by_id(&mut self, id: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>(FETCH_USER_BY_TAXI_NUM)
            .bind(id)
            .fetch_optional(&mut *self.conn)
            .await
    }
}
