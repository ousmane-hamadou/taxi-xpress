use rocket::{get, Route, routes, uri};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use data::taxi_dao;

use crate::{BASE_URL, data, XpressDB};
use crate::error::Error;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Taxi {
    pub id: Uuid,
    pub number: String,
    pub max_place: i32,
    pub available_place: i32,
    pub current_station: String,
    pub destination_station: String,
    #[serde(rename = "_links")]
    links: Links,
}

#[skip_serializing_none]
#[derive(Serialize)]
struct Links {
    #[serde(rename = "self")]
    me: String,
    bookings: Option<String>,
}

impl From<taxi_dao::Taxi> for Json<Taxi> {
    fn from(value: taxi_dao::Taxi) -> Self {
        let id = value.id;
        let me = uri!(BASE_URL, show(id)).to_string();

        Json(Taxi {
            id,
            number: value.number,
            max_place: value.max_place,
            available_place: value.available_place,
            current_station: format!("{BASE_URL}/stations/{}", value.current_station),
            destination_station: format!("{BASE_URL}/stations/{}", value.destination_station),
            links: Links { bookings: None, me },
        })
    }
}

#[get("/taxis/<id>")]
async fn show(mut conn: Connection<XpressDB>, id: Uuid) -> Result<Json<Taxi>, Error> {
    taxi_dao::from_id(&mut conn, id)
        .await?
        .map(Json::from)
        .ok_or_else(|| Error::NotFound("The taxis with id `{}` does not exists".into()))
}

pub fn routes() -> Vec<Route> {
    routes![show]
}
