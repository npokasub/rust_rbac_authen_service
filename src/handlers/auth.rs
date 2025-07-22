use crate::database::PgPool;
use crate::handlers::user::UserHandler;
use crate::utilities::error::AppError;
use crate::utilities::encryption::Encryption;
use crate::models::user::UserResponse;
use crate::middlewares::jwt;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use log::{debug, error, info};


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
    debug!("Initializing AuthHandler with expiration_seconds: {}", expiration_seconds);
    AuthHandler {
      user_handler: UserHandler::new(pool),
      jwt_secret,
      expiration_seconds,
    }
  }

  pub fn register(&self, req: &RegisterRequest) -> Result<UserResponse, AppError> {
    info!("Registering user: {}", req.username);
    debug!("Calling UserHandler to create user: {}", req.username);
    let user = self.user_handler.create(&req.username, &req.email, &req.password)?;
    info!("User registered successfully: {}", user.username);
    Ok(user.into())
  }

  pub fn login(&self, req: &LoginRequest) -> Result<LoginResponse, AppError> {
    info!("Attempting login for user: {}", req.username);
    debug!("Looking up user: {}", req.username);
    let user = self.user_handler.find_by_username(&req.username)?;
    debug!("Verifying password for user: {}", user.username);
    let is_valid = Encryption::verify_password(&req.password, &user.password_hash)?;
    if is_valid {
      debug!("Generating JWT for user: {}", user.username);
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
      .map_err(|e| {
        error!("Failed to generate JWT for user {}: {}", user.username, e);
        AppError::InternalError
      })?;
      info!("Login successful for user: {}", user.username);
      Ok(LoginResponse {
        user: user.into(),
        token,
      })
    } else {
        error!("Invalid password for user: {}", req.username);
        Err(AppError::InvalidCredentials)
    }
  }
}