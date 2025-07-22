use actix_web::{web, HttpResponse, Responder, ResponseError};
use log::{info, error};
use crate::handlers::auth::{AuthHandler, RegisterRequest, LoginRequest};

pub struct AuthRoutes;

impl AuthRoutes {
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/auth")
      .route("/register", web::post().to(Self::register))
      .route("/login", web::post().to(Self::login))
  );
}

async fn register(pool: web::Data<crate::database::PgPool>, config: web::Data<crate::config::Config>, req: web::Json<RegisterRequest>) -> impl Responder {
  info!("Processing register request for username: {}", req.username);
  let auth_handler = AuthHandler::new(&pool, config.auth.jwt_secret.clone(), config.auth.expiration_seconds);
  match auth_handler.register(&req) {
    Ok(user_response) => {
      info!("User registered successfully: {}", user_response.username);
      HttpResponse::Ok().json(user_response)
    },
    Err(e) => {
      error!("Registration failed for username: {}: {}", req.username, e);
      e.error_response()
    },
  }
}

async fn login(pool: web::Data<crate::database::PgPool>, config: web::Data<crate::config::Config>, req: web::Json<LoginRequest>) -> impl Responder {
  info!("Processing login request for username: {}", req.username);
  let auth_handler = AuthHandler::new(&pool, config.auth.jwt_secret.clone(), config.auth.expiration_seconds);
  match auth_handler.login(&req) {
    Ok(login_response) => {
      info!("User logged in successfully: {}", login_response.user.username);
      HttpResponse::Ok().json(login_response)
    },
    Err(e) => {
      error!("Login failed for username: {}: {}", req.username, e);
      e.error_response()
    },
  }
}
}