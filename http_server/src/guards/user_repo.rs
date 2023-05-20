use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_db_pools::Connection;

use crate::error::Error;
use crate::repo::user_repo::UserRepo;
use crate::XpressDB;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserRepo {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<Connection<XpressDB>>().await {
            Outcome::Success(conn) => Outcome::Success(UserRepo::new(conn)),

            e @ Outcome::Failure(_) | e @ Outcome::Forward(_) => {
                if e.is_failure() {
                    eprintln!("[user_repo_guard] {e:?}");
                }
                Outcome::Failure((Status::InternalServerError, Error::ServerError))
            }
        }
    }
}
