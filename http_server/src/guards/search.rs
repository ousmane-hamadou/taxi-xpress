use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use time::Time;
use uuid::Uuid;

use crate::error::Error;
use crate::guards::mini_search::Criteria;
use crate::guards::mini_search::MiniSearch;
use crate::models::criteria::Criteria as CriteriaModel;
use crate::repo::search_repo;
use crate::repo::search_repo::SearchRepo;

pub struct Taxi {
    pub id: Uuid,
    pub available_seats: i32,
    pub rest_time: Time,
}

impl From<search_repo::Taxi> for Taxi {
    fn from(value: search_repo::Taxi) -> Self {
        Taxi {
            id: value.id,
            available_seats: value.available_seats as i32,
            rest_time: value.rest_time,
        }
    }
}

pub struct Search {
    usecase: SearchRepo,
    pub id: Uuid,
    pub criteria: Criteria,
    available_taxis: Option<Vec<Taxi>>,
    pub selection: Option<Uuid>,
}

impl Search {
    pub async fn list_taxis(&mut self) -> Result<Option<&Vec<Taxi>>, Error> {
        if let None = self.available_taxis {
            self.available_taxis = Some(
                self.usecase
                    .list_taxis_by_criteria((&self.criteria).into())
                    .await?
                    .into_iter()
                    .map(Taxi::from)
                    .collect(),
            );
        }

        Ok(self.available_taxis.as_ref())
    }

    fn new(id: Uuid, selection: Option<Uuid>, criteria: Criteria, usecase: SearchRepo) -> Self {
        Search {
            id,
            criteria,
            selection,
            available_taxis: None,
            usecase,
        }
    }
}

impl<'r> Into<CriteriaModel<'r>> for &'r Criteria {
    fn into(self) -> CriteriaModel<'r> {
        CriteriaModel {
            origin: &self.departure_station,
            destination: &self.arrival_station,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Search {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mini_search = request.guard::<MiniSearch>().await.unwrap();

        request
            .guard::<SearchRepo>()
            .await
            .map(move |search_usecase| {
                Search::new(
                    mini_search.id,
                    mini_search.selection,
                    mini_search.criteria,
                    search_usecase,
                )
            })
    }
}
