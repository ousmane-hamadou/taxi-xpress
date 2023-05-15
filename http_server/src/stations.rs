use rocket::serde::json::Json;
use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use serde::Serialize;
use uuid::Uuid;

use crate::data::station_dao;
use crate::error::Error;
use crate::{XpressDB, BASE_URL};

#[derive(Serialize)]
struct Station {
    id: Uuid,
    name: String,
}

impl Into<Response> for Vec<Station> {
    fn into(self) -> Response {
        Response {
            stations: self,
            link: format!("{BASE_URL}/stations"),
        }
    }
}

impl From<station_dao::Station> for Station {
    fn from(value: station_dao::Station) -> Self {
        Station {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Serialize)]
struct Response {
    stations: Vec<Station>,
    #[serde(rename = "self")]
    link: String,
}

#[get("/stations")]
async fn index(mut conn: Connection<XpressDB>) -> Result<Json<Response>, Error> {
    let results = station_dao::list_all(&mut conn).await?;

    Ok(Json(
        results
            .into_iter()
            .map(|st| Station::from(st))
            .collect::<Vec<Station>>()
            .into(),
    ))
}

#[get("/stations/<id>")]
async fn show(mut conn: Connection<XpressDB>, id: Uuid) -> Result<Json<Station>, Error> {
    station_dao::from_id(&mut conn, id)
        .await?
        .map(Station::from)
        .map(|s| Json(s))
        .ok_or_else(|| Error::NotFound(format!("The stations with id `{}` does not exists", id)))
}

pub fn routes() -> Vec<Route> {
    routes![index, show]
}
