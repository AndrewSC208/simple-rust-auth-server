use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::Error as ParseError;

/// Errors codes that also have display properties. These are all the server errors that can happen
/// when calling this server.
#[derive(Debug, Display)]
pub enum ServiceError {
  #[display(fmt = "Internal Server Error")]
  InternalServerError,

  #[display(fmt = "BadRequest: {}", _0)]
  BadRequest(String),

  #[display(fmt = "Unauthorized")]
  Unauthorized,
}

/// telling what kinds of errors that will be returned for to the actix_web framework.
/// impl ResponseError trait alows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
  fn error_response(&self) -> HttpResponse {
    match self {
      ServiceError::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error, Please try later"),
      ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
      ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
    }
  }
}

/// mapping of internal system errors to external http errors

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for ServiceError {
  fn from(_: ParseError) -> ServiceError {
    ServiceError::BadRequest("Invalid UUID".into())
  }
}

// we can provide a server error if something goes wrong with the database. This is 
// an adapter from one type of `Error` (DBError) to another `Error` (ServiceError).
impl From<DBError> for ServiceError {
  fn from(error: DBError) -> ServiceError {
    // check the type of db error that was thrown
    match error {
      DBError::DatabaseError(kind, info) => {
        if let DatabaseErrorKind::UniqueViolation = kind {
          let message = info.details().unwrap_or_else(|| info.message()).to_string();
          return ServiceError::BadRequest(message);
        }

        ServiceError::InternalServerError
      }
      _ => ServiceError::InternalServerError,
    }
  }
}
