use crate::error::Error;
use crate::repo::search_repo::SearchRepo;
use crate::XpressDB;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_db_pools::Connection;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SearchRepo {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<Connection<XpressDB>>().await {
            Outcome::Success(conn) => Outcome::Success(SearchRepo::new(conn)),

            e @ Outcome::Failure(_) | e @ Outcome::Forward(_) => {
                if e.is_failure() {
                    eprintln!("[search_repo_guard] {e:?}");
                }
                Outcome::Failure((Status::InternalServerError, Error::ServerError))
            }
        }
    }
}
