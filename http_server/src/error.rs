use log::error;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::tokio::task::JoinError;
use rocket::{Request, Response};
use serde::Serialize;
use serde_with::DisplayFromStr;
use validator;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an internal server error occurred")]
    Sqlx(#[from] sqlx::Error),
    #[error("an internal server error occurred")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("{0}")]
    InvalidRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("validation error")]
    ValidationError(#[from] ValidationErrors),
    #[error("You must be authenticated")]
    NoCredentials,
    #[error("number or password is incorrect")]
    SignIncorrectData,
    #[error("an internal server error occurred")]
    InternalJoinError(#[from] JoinError),
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
            Error::Sqlx(_) | Error::Bcrypt(_) | Error::InternalJoinError(_) => {
                Status::InternalServerError
            }
            Error::SignIncorrectData | Error::ValidationError(_) | Error::InvalidRequest(_) => {
                Status::BadRequest
            }
            Error::NoCredentials => Status::Unauthorized,
            Error::NotFound(_) => Status::NotFound,
        }
    }
}
