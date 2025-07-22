use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use validator::Validate;
use crate::database::PgPool;
use crate::handlers::user::UserHandler;
use crate::models::user::UserResponse;
use log::{error, info};
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
  #[validate(length(min = 3))]
  pub username: String,
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 8))]
  pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserRequest {
  #[validate(length(min = 3))]
  pub username: Option<String>,
  #[validate(email)]
  pub email: Option<String>,
  #[validate(length(min = 8))]
  pub password: Option<String>,
}

pub struct UserRoutes;

impl UserRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/users")
        .wrap(crate::middlewares::jwt::JwtMiddleware)
        .route("", web::post().to(Self::create_user))
        .route("/{id}", web::get().to(Self::get_user))
        .route("/{id}", web::put().to(Self::update_user))
        .route("/{id}", web::delete().to(Self::delete_user)),
    );
  }

  async fn create_user(req: web::Json<CreateUserRequest>, pool: web::Data<PgPool>) -> impl Responder {
    info!("Processing create user request for username: {}", req.username);
    if let Err(e) = req.validate() {
      error!("Validation failed for user creation: {}", e);
      return HttpResponse::BadRequest().json(serde_json::json!({
        "error": format!("Validation error: {}", e)
      }));
    }
    let handler = UserHandler::new(&pool);
    match handler.create(&req.username, &req.email, &req.password) {
      Ok(user) => {
        info!("User created successfully via route: {}", user.username);
        HttpResponse::Ok().json(UserResponse::from(user))
      }
      Err(e) => {
        error!("Failed to create user {}: {}", req.username, e);
        e.error_response()
      }
    }
  }

  async fn get_user(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing get user request for ID: {}", id);
    let handler = UserHandler::new(&pool);
    match handler.find_by_id(id) {
      Ok(user) => {
        info!("User retrieved successfully: {}", id);
        HttpResponse::Ok().json(UserResponse::from(user))
      }
      Err(e) => {
        error!("Failed to retrieve user {}: {}", id, e);
        e.error_response()
      }
    }
  }

  async fn update_user(path: web::Path<Uuid>, req: web::Json<UpdateUserRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing update user request for ID: {}", id);
    if let Err(e) = req.validate() {
      error!("Validation failed for user update: {}", e);
      return HttpResponse::BadRequest().json(serde_json::json!({
        "error": format!("Validation error: {}", e)
      }));
    }
    let handler = UserHandler::new(&pool);
    match handler.update(
      id,
      req.username.as_deref(),
      req.email.as_deref(),
      req.password.as_deref(),
    ) {
      Ok(user) => {
        info!("User updated successfully: {}", id);
        HttpResponse::Ok().json(UserResponse::from(user))
      }
      Err(e) => {
        error!("Failed to update user {}: {}", id, e);
        e.error_response()
      }
    }
  }

  async fn delete_user(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing delete user request for ID: {}", id);
    let handler = UserHandler::new(&pool);
    match handler.delete(id) {
      Ok(()) => {
        info!("User deleted successfully: {}", id);
        HttpResponse::Ok().finish()
      }
      Err(e) => {
        error!("Failed to delete user {}: {}", id, e);
        e.error_response()
      }
    }
  }
}