use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use jsonwebtoken::errors::Error as JwtError;
use argon2::password_hash::Error as ArgonError;
use r2d2;
use uuid::Error as UuidError;
use chrono::ParseError;
use validator::ValidationErrors;
use std::error::Error;

#[derive(Debug, Display)]
pub enum AppError {
  #[display("Bad Request: {}", _0)]
  BadRequest(String),
  #[display("Unauthorized: {}", _0)]
  Unauthorized(String),
  #[display("Invalid Credentials")]
  InvalidCredentials,
  #[display("Forbidden")]
  Forbidden,
  #[display("Not Found")]
  NotFound,
  #[display("Conflict: {}", _0)]
  Conflict(String),
  #[display("Internal Server Error")]
  InternalError,
}

impl Error for AppError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      AppError::BadRequest(_) => None,
      AppError::Unauthorized(_) => None,
      AppError::InvalidCredentials => None,
      AppError::Forbidden => None,
      AppError::NotFound => None,
      AppError::Conflict(_) => None,
      AppError::InternalError => None,
    }
  }
}

impl ResponseError for AppError {
  fn status_code(&self) -> StatusCode {
    match self {
      AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
      AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
      AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
      AppError::Forbidden => StatusCode::FORBIDDEN,
      AppError::NotFound => StatusCode::NOT_FOUND,
      AppError::Conflict(_) => StatusCode::CONFLICT,
      AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).json(serde_json::json!({
      "error": self.to_string()
    }))
  }
}

impl From<DieselError> for AppError {
  fn from(err: DieselError) -> Self {
    match err {
      DieselError::NotFound => AppError::NotFound,
      DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
        AppError::Conflict("Duplicate entry".to_string())
      }
      _ => AppError::InternalError,
    }
  }
}

impl From<r2d2::Error> for AppError {
  fn from(_: r2d2::Error) -> Self {
    AppError::InternalError
  }
}

impl From<UuidError> for AppError {
  fn from(_: UuidError) -> Self {
    AppError::BadRequest("Invalid UUID".to_string())
  }
}

impl From<ParseError> for AppError {
  fn from(_: ParseError) -> Self {
    AppError::BadRequest("Invalid datetime format".to_string())
  }
}

impl From<JwtError> for AppError {
  fn from(err: JwtError) -> Self {
    AppError::Unauthorized(err.to_string())
  }
}

impl From<ArgonError> for AppError {
  fn from(_: ArgonError) -> Self {
    AppError::InternalError
  }
}

impl From<std::io::Error> for AppError {
  fn from(_: std::io::Error) -> Self {
    AppError::InternalError
  }
}

impl From<ValidationErrors> for AppError {
  fn from(err: ValidationErrors) -> Self {
    AppError::BadRequest(format!("Validation error: {}", err))
  }
}