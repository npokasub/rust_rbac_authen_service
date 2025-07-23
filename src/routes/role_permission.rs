use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use validator::{Validate, ValidationError};
use crate::database::PgPool;
use crate::handlers::role_permission::RolePermissionHandler;
use crate::models::role_permission::RolePermissionResponse;
use log::{error, info};
use uuid::Uuid;

// Custom validator for UUID fields
fn validate_uuid(uuid: &Uuid) -> Result<(), ValidationError> {
  // Convert UUID to string and attempt to parse it to ensure it's valid
  if uuid::Uuid::parse_str(&uuid.to_string()).is_ok() {
    Ok(())
  } else {
    Err(ValidationError::new("invalid_uuid").with_message("Invalid UUID format".into()))
  }
}

#[derive(Deserialize, Validate)]
pub struct CreateRolePermissionRequest {
  #[validate(custom(function = "validate_uuid"))]
  pub role_id: Uuid,
  #[validate(custom(function = "validate_uuid"))]
  pub permission_id: Uuid,
}

pub struct RolePermissionRoutes;

impl RolePermissionRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/role_permissions")
        .wrap(crate::middlewares::jwt::JwtMiddleware)
        .route("", web::post().to(Self::create_role_permission))
        .route("/{role_id}/{permission_id}", web::get().to(Self::get_role_permission))
        .route("/{role_id}", web::get().to(Self::get_role_permissions_by_role))
        .route("/{role_id}/{permission_id}", web::delete().to(Self::delete_role_permission)),
    );
  }

  async fn create_role_permission(req: web::Json<CreateRolePermissionRequest>, pool: web::Data<PgPool>) -> impl Responder {
    info!("Processing create role_permission request: role_id={}, permission_id={}", req.role_id, req.permission_id);
    if let Err(e) = req.0.validate() {
      error!("Validation failed for role_permission creation: {}", e);
      return HttpResponse::BadRequest().json(serde_json::json!({
        "error": format!("Validation error: {}", e)
      }));
    }
    let handler = RolePermissionHandler::new(&pool);
    match handler.create(req.role_id, req.permission_id) {
      Ok(role_permission) => {
        info!("RolePermission created successfully via route: role_id={}, permission_id={}", req.role_id, req.permission_id);
        HttpResponse::Ok().json(RolePermissionResponse::from(role_permission))
      }
      Err(e) => {
        error!("Failed to create role_permission for role_id={} and permission_id={}: {}", req.role_id, req.permission_id, e);
        e.error_response()
      }
    }
  }

  async fn get_role_permission(path: web::Path<(Uuid, Uuid)>, pool: web::Data<PgPool>) -> impl Responder {
    let (role_id, permission_id) = path.into_inner();
    info!("Processing get role_permission request: role_id={}, permission_id={}", role_id, permission_id);
    let handler = RolePermissionHandler::new(&pool);
    match handler.find_by_ids(role_id, permission_id) {
      Ok(role_permission) => {
        info!("RolePermission retrieved successfully: role_id={}, permission_id={}", role_id, permission_id);
        HttpResponse::Ok().json(RolePermissionResponse::from(role_permission))
      }
      Err(e) => {
        error!("Failed to retrieve role_permission for role_id={} and permission_id={}: {}", role_id, permission_id, e);
        e.error_response()
      }
    }
  }

  async fn get_role_permissions_by_role(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let role_id = *path;
    info!("Processing get role_permissions request for role_id={}", role_id);
    let handler = RolePermissionHandler::new(&pool);
    match handler.find_by_role_id(role_id) {
      Ok(role_permissions) => {
        info!("Retrieved {} role_permissions for role_id={}", role_permissions.len(), role_id);
        let response: Vec<RolePermissionResponse> = role_permissions.into_iter().map(RolePermissionResponse::from).collect();
        HttpResponse::Ok().json(response)
      }
      Err(e) => {
        error!("Failed to retrieve role_permissions for role_id={}: {}", role_id, e);
        e.error_response()
      }
    }
  }

  async fn delete_role_permission(path: web::Path<(Uuid, Uuid)>, pool: web::Data<PgPool>) -> impl Responder {
    let (role_id, permission_id) = path.into_inner();
    info!("Processing delete role_permission request: role_id={}, permission_id={}", role_id, permission_id);
    let handler = RolePermissionHandler::new(&pool);
    match handler.delete(role_id, permission_id) {
      Ok(()) => {
        info!("RolePermission deleted successfully: role_id={}, permission_id={}", role_id, permission_id);
        HttpResponse::Ok().finish()
      }
      Err(e) => {
        error!("Failed to delete role_permission for role_id={} and permission_id={}: {}", role_id, permission_id, e);
        e.error_response()
      }
    }
  }
}