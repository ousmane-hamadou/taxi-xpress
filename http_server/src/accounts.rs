use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use rocket::{get, post, routes, uri, Route};
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

use crate::data::taxi_dao;
use crate::data::user_account_dao;
use crate::error::Error;
use crate::utils::password;
use crate::{XpressDB, BASE_URL};

const AUTH_COOKIE: &'static str = "AUTH_COOKIE";

#[skip_serializing_none]
#[derive(Serialize)]
struct Links {
    #[serde(rename = "self")]
    author: String,
    bookings: Option<String>,
}

#[derive(Serialize)]
struct User {
    id: Uuid,
    links: Links,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Taxi {
    pub number: String,
    pub max_place: i32,
    pub current_station: Uuid,
    pub destination_station: Uuid,
}

#[derive(Deserialize)]
struct NewUser {
    taxi: Taxi,
}

impl Into<user_account_dao::UserDto> for NewUser {
    fn into(self) -> user_account_dao::UserDto {
        let t = self.taxi;

        let taxi = taxi_dao::Taxi {
            id: Uuid::new_v4(),
            number: t.number.to_lowercase(),
            max_place: t.max_place,
            available_place: t.max_place,
            current_station: t.current_station,
            destination_station: t.destination_station,
        };

        user_account_dao::UserDto {
            taxi,
            id: Uuid::new_v4(),
            password: String::new(),
        }
    }
}

impl From<user_account_dao::User> for Json<User> {
    fn from(value: user_account_dao::User) -> Self {
        Json(User {
            id: value.id,
            links: Links {
                author: "".into(),
                bookings: None,
            },
        })
    }
}

#[derive(Debug, Deserialize, Validate)]
struct UserData<'r> {
    number: &'r str,
    #[validate(length(min = 8, max = 125))]
    password: &'r str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AccountLinks<'r> {
    sign_in: HashMap<&'r str, String>,
}

#[post("/accounts/admin/create-user", data = "<data>")]
async fn create_user(mut conn: Connection<XpressDB>, data: Json<NewUser>) -> Result<Status, Error> {
    let password = password::gen_password().await?;
    let mut user: user_account_dao::UserDto = data.0.into();
    user.password = password.hashed;

    user_account_dao::insert(&mut conn, &user).await?;

    Ok(Status::Created)
}

pub fn routes() -> Vec<Route> {
    routes![create_user]
}
