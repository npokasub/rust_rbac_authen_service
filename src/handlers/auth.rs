use crate::database::PgPool;
use crate::handlers::user::UserHandler;
use crate::utilities::error::AppError;
use crate::utilities::encryption::Encryption;
use crate::models::user::UserResponse;
use crate::middlewares::jwt;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};


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
  pub token: String,
}

pub struct AuthHandler<'a> {
  user_handler: UserHandler<'a>,
  jwt_secret: String,
  expiration_seconds: i64,
}

impl<'a> AuthHandler<'a> {
  pub fn new(pool: &'a PgPool, jwt_secret: String, expiration_seconds: i64) -> Self {
    AuthHandler {
      user_handler: UserHandler::new(pool),
      jwt_secret,
      expiration_seconds,
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
        let exp = (Utc::now() + Duration::seconds(self.expiration_seconds)).timestamp() as usize;
        let claims = jwt::Claims {
          sub: user.id.to_string(),
          exp,
        };
        let token = encode(
          &Header::default(),
          &claims,
          &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::InternalError)?;
        Ok(LoginResponse {
            user: user.into(),
            token,
      })
    } else {
      Err(AppError::InvalidCredentials)
    }
  }
}