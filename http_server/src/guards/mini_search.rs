use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::Error;

pub struct MiniSearch {
    pub id: Uuid,
    pub criteria: Criteria,
    pub selection: Option<Uuid>,
}

#[derive(Deserialize)]
struct MiniSearchDto {
    pub criteria: Criteria,
    pub selection: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct Criteria {
    pub departure_station: Uuid,
    pub arrival_station: Uuid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MiniSearch {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let res = request.param::<Uuid>(1).unwrap().map(|search_id| {
            let id = search_id.to_string();
            
            request
                .cookies()
                .get_private(&id)
                .map(|c| serde_json::from_str::<MiniSearchDto>(c.value()))
                .and_then(|ms| {
                    if let Err(_) = ms {
                        return None;
                    }

                    Some(
                        ms.map(|ms| MiniSearch {
                            id: search_id,
                            criteria: ms.criteria,
                            selection: ms.selection,
                        })
                        .unwrap(),
                    )
                })
        });

        match res {
            Err(_) => Outcome::Failure((
                Status::NotFound,
                Error::UnknownSearch(request.param(1).unwrap().unwrap()),
            )),
            Ok(r) => match r {
                None => Outcome::Failure((
                    Status::NotFound,
                    Error::UnknownSearch(request.param(1).unwrap().unwrap()),
                )),
                Some(ms) => Outcome::Success(ms),
            },
        }
    }
}
