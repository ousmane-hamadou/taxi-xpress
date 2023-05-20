use crate::XpressDB;
use rocket_db_pools::Connection;
use sqlx::Acquire;
use uuid::Uuid;

const INSERT_USER: &'static str =
    "INSERT INTO user_accounts(id, full_name, password) VALUES ($1, $2, $3)";
const INSERT_TAXI: &'static str =
    "INSERT INTO taxis(id, number, number_of_seats) VALUES ($1, $2, $3)";

pub struct Taxi<'r> {
    pub id: Uuid,
    pub number: &'r str,
    pub number_of_seats: i32,
}

pub struct User<'r> {
    pub id: &'r str,
    pub full_name: &'r str,
    pub password: &'r str,
    pub taxi: Taxi<'r>,
}

pub struct AdminRepo {
    conn: Connection<XpressDB>,
}

impl AdminRepo {
    pub fn new(conn: Connection<XpressDB>) -> Self {
        AdminRepo { conn }
    }

    pub async fn insert_user(&mut self, user: User<'_>) -> sqlx::Result<()> {
        let mut tx = self.conn.begin().await?;

        sqlx::query(INSERT_TAXI)
            .bind(user.taxi.id)
            .bind(user.taxi.number)
            .bind(user.taxi.number_of_seats)
            .execute(&mut *tx)
            .await?;

        sqlx::query(INSERT_USER)
            .bind(user.id)
            .bind(user.full_name)
            .bind(user.password)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
