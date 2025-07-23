use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use validator::{Validate, ValidationError};
use crate::database::PgPool;
use crate::handlers::user_role::UserRoleHandler;
use crate::models::user_role::UserRoleResponse;
use log::{error, info};
use uuid::Uuid;

// Custom validator for UUID fields (reused from role_permission.rs)
fn validate_uuid(uuid: &Uuid) -> Result<(), ValidationError> {
  if uuid::Uuid::parse_str(&uuid.to_string()).is_ok() {
    Ok(())
  } else {
    Err(ValidationError::new("invalid_uuid").with_message("Invalid UUID format".into()))
  }
}

#[derive(Deserialize, Validate)]
pub struct CreateUserRoleRequest {
  #[validate(custom(function = "validate_uuid"))]
  pub user_id: Uuid,
  #[validate(custom(function = "validate_uuid"))]
  pub role_id: Uuid,
}

pub struct UserRoleRoutes;

impl UserRoleRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/user_roles")
        .wrap(crate::middlewares::jwt::JwtMiddleware)
        .route("", web::post().to(Self::create_user_role))
        .route("/{user_id}/{role_id}", web::get().to(Self::get_user_role))
        .route("/{user_id}", web::get().to(Self::get_user_roles_by_user))
        .route("/{user_id}/{role_id}", web::delete().to(Self::delete_user_role)),
    );
  }

  async fn create_user_role(req: web::Json<CreateUserRoleRequest>, pool: web::Data<PgPool>) -> impl Responder {
    info!("Processing create user_role request: user_id={}, role_id={}", req.user_id, req.role_id);
    if let Err(e) = req.0.validate() {
      error!("Validation failed for user_role creation: {}", e);
      return HttpResponse::BadRequest().json(serde_json::json!({
        "error": format!("Validation error: {}", e)
      }));
    }
    let handler = UserRoleHandler::new(&pool);
    match handler.create(req.user_id, req.role_id) {
      Ok(user_role) => {
        info!("UserRole created successfully via route: user_id={}, role_id={}", req.user_id, req.role_id);
        HttpResponse::Ok().json(UserRoleResponse::from(user_role))
      }
      Err(e) => {
        error!("Failed to create user_role for user_id={} and role_id={}: {}", req.user_id, req.role_id, e);
        e.error_response()
      }
    }
  }

  async fn get_user_role(path: web::Path<(Uuid, Uuid)>, pool: web::Data<PgPool>) -> impl Responder {
    let (user_id, role_id) = path.into_inner();
    info!("Processing get user_role request: user_id={}, role_id={}", user_id, role_id);
    let handler = UserRoleHandler::new(&pool);
    match handler.find_by_ids(user_id, role_id) {
      Ok(user_role) => {
        info!("UserRole retrieved successfully: user_id={}, role_id={}", user_id, role_id);
        HttpResponse::Ok().json(UserRoleResponse::from(user_role))
      }
      Err(e) => {
        error!("Failed to retrieve user_role for user_id={} and role_id={}: {}", user_id, role_id, e);
        e.error_response()
      }
    }
  }

  async fn get_user_roles_by_user(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let user_id = *path;
    info!("Processing get user_roles request for user_id={}", user_id);
    let handler = UserRoleHandler::new(&pool);
    match handler.find_by_user_id(user_id) {
      Ok(user_roles) => {
        info!("Retrieved {} user_roles for user_id={}", user_roles.len(), user_id);
        let response: Vec<UserRoleResponse> = user_roles.into_iter().map(UserRoleResponse::from).collect();
        HttpResponse::Ok().json(response)
      }
      Err(e) => {
        error!("Failed to retrieve user_roles for user_id={}: {}", user_id, e);
        e.error_response()
      }
    }
  }

  async fn delete_user_role(path: web::Path<(Uuid, Uuid)>, pool: web::Data<PgPool>) -> impl Responder {
    let (user_id, role_id) = path.into_inner();
    info!("Processing delete user_role request: user_id={}, role_id={}", user_id, role_id);
    let handler = UserRoleHandler::new(&pool);
    match handler.delete(user_id, role_id) {
      Ok(()) => {
        info!("UserRole deleted successfully: user_id={}, role_id={}", user_id, role_id);
        HttpResponse::Ok().finish()
      }
      Err(e) => {
        error!("Failed to delete user_role for user_id={} and role_id={}: {}", user_id, role_id, e);
        e.error_response()
      }
    }
  }
}