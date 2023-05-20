use std::collections::HashMap;

use rocket::http::{Cookie, CookieJar};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{get, post, routes, uri, Route};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::controllers::trips;
use crate::error::Error;
use crate::models::taxi_owner::TaxiOwner;
use crate::repo::admin_repo;
use crate::repo::admin_repo::AdminRepo;
use crate::repo::user_repo::{User, UserRepo};
use crate::utils::{password, Link};
use crate::{BASE_URL, TAXI_OWNER_COOKIE_KEY};

// taxi-owner
#[derive(Deserialize)]
struct UserCredentials<'r> {
    number: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
pub struct TaxiOwnerResource {
    id: String,
    full_name: String,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Into<TaxiOwner> for User {
    fn into(self) -> TaxiOwner {
        TaxiOwner {
            id: self.id,
            taxi_id: self.taxi_id,
            name: self.full_name,
        }
    }
}

impl Into<TaxiOwnerResource> for TaxiOwner {
    fn into(self) -> TaxiOwnerResource {
        TaxiOwnerResource {
            id: self.id,
            full_name: self.name,
            links: taxi_owner_resource_links(),
        }
    }
}

// admin
#[derive(Deserialize, Validate)]
struct Taxi<'r> {
    #[validate(length(min = 8, max = 8))]
    number: &'r str,
    #[validate(range(min = 4, max = 6))]
    number_of_seats: u8,
}

#[derive(Deserialize, Validate)]
struct UserDto<'r> {
    #[validate(length(min = 4, max = 120))]
    name: &'r str,
    taxi: Taxi<'r>,
}

impl<'r> Into<admin_repo::User<'r>> for UserDto<'r> {
    fn into(self) -> admin_repo::User<'r> {
        admin_repo::User {
            id: self.taxi.number,
            full_name: self.name,
            password: "",
            taxi: admin_repo::Taxi {
                id: Uuid::new_v4(),
                number_of_seats: self.taxi.number_of_seats as i32,
                number: self.taxi.number,
            },
        }
    }
}

// controllers

#[post("/accounts/users/login", format = "json", data = "<credentials>")]
async fn user_login(
    credentials: Json<UserCredentials<'_>>,
    cookies: &CookieJar<'_>,
    mut user_repo: UserRepo,
) -> Result<Json<TaxiOwnerResource>, Error> {
    match user_repo.fetch_by_id(credentials.number).await? {
        None => Err(Error::InvalidCredentials(credentials.number.into())),
        Some(user) => {
            if password::verify(credentials.password, &user.password).await? {
                let user: TaxiOwner = user.into();

                let cookie =
                    Cookie::new(TAXI_OWNER_COOKIE_KEY, serde_json::to_string(&user).unwrap());
                cookies.add_private(cookie);

                return Ok(Json(user.into()));
            }
            Err(Error::InvalidCredentials(credentials.number.into()))
        }
    }
}

#[get("/accounts/users")]
async fn user_account(user: Result<TaxiOwner, Error>) -> Result<Json<TaxiOwnerResource>, Error> {
    let user = user?;

    Ok(Json(user.into()))
}

#[post("/admin/create-taxi", data = "<user>")]
async fn create_taxi(
    user: Json<UserDto<'_>>,
    mut admin_repo: AdminRepo,
) -> Result<status::Created<&'static str>, Error> {
    user.validate()?;
    user.taxi.validate()?;

    let mut user: admin_repo::User = user.0.into();
    let password = password::gen_password().await.unwrap();
    user.password = &password.hashed;

    admin_repo.insert_user(user).await?;
    Ok(status::Created::new(
        uri!(BASE_URL, user_account()).to_string(),
    ))
}

fn taxi_owner_resource_links() -> HashMap<&'static str, Link> {
    HashMap::from([
        (
            "self",
            Link {
                href: uri!(BASE_URL, user_account()).to_string(),
            },
        ),
        (
            "start-journey",
            Link {
                href: uri!(BASE_URL, trips::start_trip()).to_string(),
            },
        ),
    ])
}

pub fn routes() -> Vec<Route> {
    routes![user_account, user_login, create_taxi]
}
