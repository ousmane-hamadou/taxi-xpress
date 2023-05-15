use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::uri::Absolute;
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::{get, routes, uri, Build, Request, Response, Rocket};
use rocket_db_pools::Database;
use serde::Serialize;
use std::collections::HashMap;

mod accounts;
mod data;
mod error;
mod password;
mod stations;
mod taxis;

pub const BASE_URL: Absolute<'static> = uri!("http://localhost:8000");

pub const AUTH_COOKIE: &'static str = "AUTHN-COOKIE";

#[derive(Database)]
#[database("xpress")]
pub struct XpressDB(sqlx::PgPool);

#[derive(Serialize)]
struct Link {
    href: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    endpoints: HashMap<&'static str, Link>,
}

#[get("/")]
fn endpoints() -> Json<Endpoints> {
    let mut endpoints = HashMap::new();

    endpoints.insert(
        "stations",
        Link {
            href: uri!(BASE_URL, stations::index()).to_string(),
        },
    );
    Json(Endpoints { endpoints })
}

pub fn server() -> Rocket<Build> {
    rocket::build()
        .attach(XpressDB::init())
        .attach(CORS)
        .mount("/", routes![endpoints])
        .mount("/", taxis::routes())
        .mount("/", stations::routes())
        .mount("/", accounts::routes())
}
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
