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
use log::error;

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
  #[display("Not Found: {}", _0)]
  NotFound(String),
  #[display("Conflict: {}", _0)]
  Conflict(String),
  #[display("Database Error: {}", _0)]
  DatabaseError(String),
  #[display("Connection Error: {}", _0)]
  ConnectionError(String),
  #[display("Hashing Error: {}", _0)]
  HashingError(String),
  #[display("JWT Error: {}", _0)]
  JwtError(String),
}

impl Error for AppError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      AppError::BadRequest(_) => None,
      AppError::Unauthorized(_) => None,
      AppError::InvalidCredentials => None,
      AppError::Forbidden => None,
      AppError::NotFound(_) => None,
      AppError::Conflict(_) => None,
      AppError::DatabaseError(_) => None,
      AppError::ConnectionError(_) => None,
      AppError::HashingError(_) => None,
      AppError::JwtError(_) => None,
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
      AppError::NotFound(_) => StatusCode::NOT_FOUND,
      AppError::Conflict(_) => StatusCode::CONFLICT,
      AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::ConnectionError(_) => StatusCode::SERVICE_UNAVAILABLE,
      AppError::HashingError(_) => StatusCode::BAD_REQUEST,
      AppError::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_response(&self) -> HttpResponse {
    let error_message = self.to_string();
    error!("Returning error response: {}", error_message);
    HttpResponse::build(self.status_code()).json(serde_json::json!({
      "error": error_message
    }))
  }
}

impl From<DieselError> for AppError {
  fn from(err: DieselError) -> Self {
    match err {
      DieselError::NotFound => {
        error!("Diesel error: Resource not found");
        AppError::NotFound("Resource not found".to_string())
      }
      DieselError::DatabaseError(kind, info) => {
        match kind {
          DatabaseErrorKind::UniqueViolation => {
            match info.constraint_name() {
              Some("users_username_key") => {
                error!("Unique violation on users_username_key: {}", info.message());
                AppError::Conflict("User already exists".to_string())
              }
              Some("users_email_key") => {
                error!("Unique violation on users_email_key: {}", info.message());
                AppError::Conflict("User already exists".to_string())
              }
              Some("roles_name_key") => {
                error!("Unique violation on roles_name_key: {}", info.message());
                AppError::Conflict("Role already exists".to_string())
              }
              Some("permissions_name_key") => {
                error!("Unique violation on permissions_name_key: {}", info.message());
                AppError::Conflict("Permission already exists".to_string())
              }
              Some("user_roles_user_id_role_id_key") => {
                error!("Unique violation on user_roles_user_id_role_id_key: {}", info.message());
                AppError::Conflict("Duplicate entry for constraint: user_roles_user_id_role_id_key".to_string())
              }
              Some("role_permissions_role_id_permission_id_key") => {
                error!("Unique violation on role_permissions_role_id_permission_id_key: {}", info.message());
                AppError::Conflict("Duplicate entry for constraint: role_permissions_role_id_permission_id_key".to_string())
              }
              Some(constraint) => {
                error!("Unique violation on constraint {}: {}", constraint, info.message());
                AppError::Conflict(format!("Duplicate entry for constraint: {}", constraint))
              }
              None => {
                error!("Unique violation with no constraint name: {}", info.message());
                AppError::Conflict("Duplicate entry".to_string())
              }
            }
          }
          DatabaseErrorKind::ForeignKeyViolation => {
            error!("Foreign key violation: {}", info.message());
            AppError::BadRequest(format!("Invalid reference: {}", info.message()))
          }
          DatabaseErrorKind::SerializationFailure => {
            error!("Serialization failure: {}", info.message());
            AppError::DatabaseError(format!("Transaction conflict: {}", info.message()))
          }
          _ => {
            error!("Database error of kind {:?}: {}", kind, info.message());
            AppError::DatabaseError(format!("Database error: {}", info.message()))
          }
        }
      }
      DieselError::QueryBuilderError(err) => {
        error!("Query builder error: {}", err);
        AppError::BadRequest(format!("Invalid query: {}", err))
      }
      DieselError::DeserializationError(err) => {
        error!("Deserialization error: {}", err);
        AppError::BadRequest(format!("Invalid data format: {}", err))
      }
      DieselError::SerializationError(err) => {
        error!("Serialization error: {}", err);
        AppError::DatabaseError(format!("Serialization error: {}", err))
      }
      DieselError::RollbackErrorOnCommit { rollback_error, commit_error } => {
        error!("Rollback error on commit: rollback={}, commit={}", rollback_error, commit_error);
        AppError::DatabaseError(format!("Transaction failed: rollback={}, commit={}", rollback_error, commit_error))
      }
      DieselError::RollbackTransaction => {
        error!("Transaction rolled back");
        AppError::DatabaseError("Transaction rolled back".to_string())
      }
      DieselError::AlreadyInTransaction => {
        error!("Already in transaction");
        AppError::DatabaseError("Already in transaction".to_string())
      }
      DieselError::NotInTransaction => {
        error!("Not in transaction");
        AppError::DatabaseError("Not in transaction".to_string())
      }
      DieselError::BrokenTransactionManager => {
        error!("Broken transaction manager");
        AppError::DatabaseError("Transaction manager error".to_string())
      }
      _ => {
        error!("Unhandled Diesel error: {:?}", err);
        AppError::DatabaseError(format!("Unknown database error: {:?}", err))
      }
    }
  }
}

impl From<r2d2::Error> for AppError {
  fn from(err: r2d2::Error) -> Self {
    error!("Connection pool error: {}", err);
    AppError::ConnectionError(format!("Failed to access database: {}", err))
  }
}

impl From<UuidError> for AppError {
  fn from(err: UuidError) -> Self {
    error!("UUID error: {}", err);
    AppError::BadRequest(format!("Invalid UUID: {}", err))
  }
}

impl From<ParseError> for AppError {
  fn from(err: ParseError) -> Self {
    error!("Parse error: {}", err);
    AppError::BadRequest(format!("Invalid datetime format: {}", err))
  }
}

impl From<JwtError> for AppError {
  fn from(err: JwtError) -> Self {
    error!("JWT error: {}", err);
    AppError::JwtError(format!("Failed to process JWT: {}", err))
  }
}

impl From<ArgonError> for AppError {
  fn from(err: ArgonError) -> Self {
    error!("Argon2 error: {}", err);
    AppError::HashingError(format!("Password hashing error: {}", err))
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    error!("IO error: {}", err);
    AppError::BadRequest(format!("IO error: {}", err))
  }
}

impl From<ValidationErrors> for AppError {
  fn from(err: ValidationErrors) -> Self {
    error!("Validation error: {}", err);
    AppError::BadRequest(format!("Validation error: {}", err))
  }
}