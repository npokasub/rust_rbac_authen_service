use crate::database::PgPool;
use crate::handlers::user::UserHandler;
use crate::utilities::error::AppError;
use crate::utilities::encryption::Encryption;
use crate::models::user::UserResponse;
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct RegisterRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
  pub username: String,
  pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
  pub user: UserResponse,
}

pub struct AuthHandler<'a> {
  user_handler: UserHandler<'a>,
}

impl<'a> AuthHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    AuthHandler {
      user_handler: UserHandler::new(pool),
    }
  }

  pub fn register(&self, req: &RegisterRequest) -> Result<UserResponse, AppError> {
    let user = self.user_handler.create(&req.username, &req.email, &req.password)?;
    Ok(user.into())
  }

  pub fn login(&self, req: &LoginRequest) -> Result<LoginResponse, AppError> {
    let user = self.user_handler.find_by_username(&req.username)?;
    let is_valid = Encryption::verify_password(&req.password, &user.password_hash)?;
    if is_valid {
      Ok(LoginResponse {
        user: user.into()
      })
    } else {
      Err(AppError::InvalidCredentials)
    }
  }
}