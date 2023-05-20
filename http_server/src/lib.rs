use std::path::PathBuf;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::uri::Absolute;
use rocket::http::{Header, Status};
use rocket::{uri, Build, Request, Response, Rocket, options, routes};
use rocket_db_pools::Database;
use crate::error::Error;

mod accounts;
mod controllers;
mod data;
mod error;
mod guards;
mod models;
mod repo;
mod utils;

pub const BASE_URL: Absolute<'static> = uri!("http://localhost:8000");
pub const SEARCH_COOKIE: &'static str = "search-cookie";
pub const TAXI_OWNER_COOKIE_KEY: &'static str = "taxi-owner";
pub const MIN_ELAPSED_TIME: i64 = 5; // in minutes

#[derive(Database)]
#[database("xpress")]
pub struct XpressDB(sqlx::PgPool);

pub fn server() -> Rocket<Build> {
    rocket::build()
        .attach(XpressDB::init())
        .attach(CORS)
        .mount("/", controllers::searches_routes())
        .mount("/", controllers::stations_routes())
        .mount("/", controllers::trips_routes())
        .mount("/", controllers::taxis_routes())
        .mount("/", controllers::accounts_routes())
        .mount("/", routes![for_cors])
}

#[options("/<p..>")]
fn for_cors(p: PathBuf)-> Result<(Status, &'static str), Error> {
    Ok((Status::Ok, ""))
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
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3000"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Cookie"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
