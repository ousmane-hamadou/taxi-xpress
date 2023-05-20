use crate::error::Error;
use crate::models::taxi_owner::TaxiOwner;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use crate::TAXI_OWNER_COOKIE_KEY;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TaxiOwner {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let owner = request
            .cookies()
            .get_private(TAXI_OWNER_COOKIE_KEY)
            .map(|cookie| serde_json::from_str::<TaxiOwner>(cookie.value()));

        match owner {
            None => Outcome::Failure((Status::Unauthorized, Error::NoCredentials)),
            Some(owner) => match owner {
                Err(_) => Outcome::Failure((Status::Unauthorized, Error::NoCredentials)),
                Ok(u) => Outcome::Success(u),
            },
        }
    }
}
