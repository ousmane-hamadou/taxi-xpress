use std::collections::HashMap;

use rocket::http::{Cookie, CookieJar, Header, Status};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{get, post, routes, uri, Responder, Route};
use rocket::response::status;
use serde::Serialize;
use serde_with::skip_serializing_none;
use time::Time;
use uuid::Uuid;
use validator::Validate;

use crate::controllers::stations;
use crate::controllers::taxis;
use crate::error::Error;
use crate::guards::mini_search::MiniSearch;
use crate::guards::search::Search as SearchDomain;
use crate::guards::{mini_search, search};
use crate::utils::{get_uuid_from_station_url, Link};
use crate::BASE_URL;

// criteria
#[derive(Validate, Deserialize, Serialize)]
struct Criteria {
    #[validate(url)]
    departure_station: String,
    #[validate(url)]
    arrival_station: String,
}

#[derive(Serialize)]
struct CriteriaForCookie {
    departure_station: Uuid,
    arrival_station: Uuid,
}

impl<'r> Into<Criteria> for mini_search::Criteria {
    fn into(self) -> Criteria {
        Criteria {
            arrival_station: uri!(BASE_URL, stations::show(self.arrival_station)).to_string(),
            departure_station: uri!(BASE_URL, stations::show(self.departure_station)).to_string(),
        }
    }
}

impl<'r> Into<CriteriaForCookie> for mini_search::Criteria {
    fn into(self) -> CriteriaForCookie {
        CriteriaForCookie {
            arrival_station: self.arrival_station,
            departure_station: self.departure_station,
        }
    }
}

impl Into<CriteriaForCookie> for Criteria {
    fn into(self) -> CriteriaForCookie {
        CriteriaForCookie {
            departure_station: get_uuid_from_station_url(&self.departure_station),
            arrival_station: get_uuid_from_station_url(&self.arrival_station),
        }
    }
}

impl Into<Criteria> for CriteriaForCookie {
    fn into(self) -> Criteria {
        Criteria {
            arrival_station: uri!(BASE_URL, stations::show(self.arrival_station)).to_string(),
            departure_station: uri!(BASE_URL, stations::show(self.departure_station)).to_string(),
        }
    }
}

// searches
#[skip_serializing_none]
#[derive(Serialize)]
struct Search {
    id: Uuid,
    criteria: CriteriaForCookie,
    selection: Option<Uuid>,
}

impl Search {
    fn new(id: Uuid, criteria: CriteriaForCookie, selection: Option<Uuid>) -> Self {
        Search {
            id,
            criteria,
            selection,
        }
    }
}

impl<'r> Into<Cookie<'r>> for &Search {
    fn into(self) -> Cookie<'r> {
        let payload = serde_json::to_string(self).unwrap();

        Cookie::build(self.id.to_string(), payload)
            .http_only(true)
            .path("/")
            .finish()
    }
}

impl<'r> From<MiniSearch> for Search {
    fn from(value: MiniSearch) -> Self {
        Search {
            id: value.id,
            criteria: value.criteria.into(),
            selection: value.selection,
        }
    }
}

impl<'r> Into<SearchResource> for Search {
    fn into(self) -> SearchResource {
        let mut links = search_resource_links(&self.id);
        if let Some(id) = self.selection {
            links.insert(
                "selected_taxi",
                Link {
                    href: uri!(BASE_URL, taxis::show(id)).to_string(),
                },
            );
        }

        SearchResource {
            id: self.id,
            criteria: self.criteria.into(),
            links: search_resource_links(&self.id),
        }
    }
}

#[derive(Serialize)]
struct SearchResource {
    id: Uuid,
    criteria: Criteria,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

#[derive(Responder)]
struct PerformSearchResponse<'r> {
    inner: Json<SearchResource>,
    header: Header<'r>,
}

// taxis

#[derive(Serialize)]
struct Taxi {
    number: String,
    available_seats: u8,
    departure_time: Time,
    model: String,
    journey_id: Uuid
}

impl From<&search::Taxi> for Taxi {
    fn from(value: &search::Taxi) -> Self {
        Taxi {
            number: value.number.to_uppercase(),
            available_seats: value.available_seats as u8,
            departure_time: value.rest_time,
            model: value.model.clone(),
            journey_id: value.journey_id
        }
    }
}

#[derive(Serialize)]
struct TaxiResource {
    taxis: Vec<Taxi>,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

// seats

#[derive(Validate, Deserialize)]
struct Seats {
    #[validate(range(min=1))]
    number: u8
}

// controllers

#[post("/searches", format = "application/json", data = "<criteria>")]
async fn perform_search(
    criteria: Json<Criteria>,
    cookies: &CookieJar<'_>,
) -> Result<(Status, PerformSearchResponse<'static>), Error> {
    criteria.validate()?;

    let s = Search::new(Uuid::new_v4(), criteria.0.into(), None);

    cookies.add_private((&s).into());
    Ok((
        Status::Created,
        PerformSearchResponse {
            inner: Json(s.into()),
            header: Header::new("Access-Control-Expose-Headers", "Set-Cookie"),
        },
    ))
}

#[get("/searches/<_id>")]
async fn retrieve_search(
    _id: Uuid,
    search_domain: Result<MiniSearch, Error>,
) -> Result<Json<SearchResource>, Error> {
    let search_domain = search_domain?;
    let search = Search::from(search_domain);
    let resp = search.into();

    Ok(Json(resp))
}

#[get("/searches/<id>/taxis")]
async fn taxis_from_search(
    id: Uuid,
    _ms: MiniSearch,
    search_domain: Result<SearchDomain, Error>,
) -> Result<Json<TaxiResource>, Error> {
    let mut search_domain = search_domain?;

    search_domain
        .list_taxis()
        .await?
        .and_then(|taxis| Some(taxis.iter().map(Taxi::from).collect::<Vec<Taxi>>()))
        .and_then(|taxis| {
            Some(Json(TaxiResource {
                taxis,
                links: taxis_resource_links(id),
            }))
        })
        .ok_or_else(|| Error::ServerError)
}

#[post("/searches/<_id>/taxis/<taxi_id>/journey/<journey_id>/select", data="<seats>")]
async fn select_taxis(
    _id: Uuid,
    taxi_id: Uuid,
    seats: Json<Seats>,
    mut mini_search: MiniSearch,
    journey_id: Uuid,
    search_domain: Result<SearchDomain, Error>,
    cookies: &CookieJar<'_>
) -> Result<(Status, PerformSearchResponse<'static>), Error> {
    seats.validate()?;
    let mut search_domain = search_domain?;

    match search_domain.has_seats_form_book(&journey_id, seats.number).await? {
        false => Err(Error::NoMoreSeats),
        _ => {
            mini_search.selection = Some(taxi_id);

            let s = Search::from(mini_search);

            cookies.add_private((&s).into());

            Ok((
                Status::Created,
                PerformSearchResponse {
                    inner: Json(s.into()),
                    header: Header::new("Access-Control-Expose-Headers", "Set-Cookie"),
                },
            ))
        }
    }
}

#[get("/searches/<id>/selection")]
async fn retrieve_selection(id: Uuid) {
    todo!()
}

fn taxis_resource_links(search_id: Uuid) -> HashMap<&'static str, Link> {
    HashMap::from([
        (
            "self",
            Link {
                href: uri!(BASE_URL, taxis_from_search(search_id)).to_string(),
            },
        ),
        (
            "search",
            Link {
                href: uri!(BASE_URL, retrieve_search(search_id)).to_string(),
            },
        ),
    ])
}

fn search_resource_links(id: &Uuid) -> HashMap<&'static str, Link> {
    HashMap::from([
        (
            "self",
            Link {
                href: uri!(BASE_URL, retrieve_search(id)).to_string(),
            },
        ),
        (
            "taxis",
            Link {
                href: uri!(BASE_URL, taxis_from_search(id)).to_string(),
            },
        ),
    ])
}

pub fn routes() -> Vec<Route> {
    routes![
        perform_search,
        retrieve_search,
        retrieve_selection,
        taxis_from_search,
        select_taxis
    ]
}
