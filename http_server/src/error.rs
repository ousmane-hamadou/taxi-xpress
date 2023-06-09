use log::error;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::tokio::task;
use rocket::{Request, Response};
use serde::Serialize;
use serde_json::Error as SerdeJsonError;
use serde_with::DisplayFromStr;
use validator;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // server error
    #[error("an internal server error occurred")]
    Sqlx(#[from] sqlx::Error),
    #[error("an internal server error occurred")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("an internal server error occurred")]
    JsonParser(#[from] SerdeJsonError),
    #[error("an internal server error occurred")]
    JoinError(#[from] task::JoinError),
    #[error("an internal server error occurred")]
    ServerError,

    // resource not found
    #[error("unknown search id `{0}`")]
    UnknownSearch(String),
    #[error("{0}")]
    NotFound(String),
    #[error("unknown journey id `{0}`")]
    UnknownJourney(String),

    // not processable request
    #[error("validation error")]
    ValidationError(#[from] ValidationErrors),
    #[error("number `{0}` or password is incorrect")]
    InvalidCredentials(String),
    #[error("invalid value `{0}` for `act` property")]
    InvalidAccountAction(String),
    #[error("there is no more seats")]
    NoMoreSeats,

    // authn/auth error
    #[error("You must be authenticated")]
    NoCredentials,
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        #[serde_with::serde_as]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ErrorResponse<'a> {
            error: &'static str,
            #[serde_as(as = "DisplayFromStr")]
            error_description: &'a Error,
        }
        eprintln!("{self:?}");

        let error = match self.status_code() {
            Status { code: 404 } => "not_found",
            Status { code: 400 } => "invalid_request",
            Status { code: 401 } => "no_credentials",
            Status { .. } => "server_error",
        };

        error!(target: "API error: ", "{self}");

        let json = Json(ErrorResponse {
            error,
            error_description: &self,
        });

        Response::build_from(json.respond_to(request)?)
            .status(self.status_code())
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

impl Error {
    fn status_code(&self) -> Status {
        match self {
            Error::Sqlx(_)
            | Error::Bcrypt(_)
            | Error::JoinError(_)
            | Error::JsonParser(_)
            | Error::ServerError => Status::InternalServerError,
            Error::InvalidCredentials(_)
            | Error::ValidationError(_)
            | Error::InvalidAccountAction(_)
            | Error::NoMoreSeats => Status::BadRequest,
            Error::NoCredentials => Status::Unauthorized,
            Error::NotFound(_) | Error::UnknownSearch(_) | Error::UnknownJourney(_) => Status::NotFound,
        }
    }
}
