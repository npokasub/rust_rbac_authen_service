use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use validator::Validate;
use crate::database::PgPool;
use crate::handlers::permission::PermissionHandler;
use crate::models::permission::PermissionResponse;
use log::{error, info};
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct CreatePermissionRequest {
  #[validate(length(min = 2))]
  pub name: String,
  #[validate(length(min = 1))]
  pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePermissionRequest {
  #[validate(length(min = 2))]
  pub name: Option<String>,
  #[validate(length(min = 1))]
  pub description: Option<String>,
}

pub struct PermissionRoutes;

impl PermissionRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/permissions")
        .wrap(crate::middlewares::jwt::JwtMiddleware)
        .route("", web::post().to(Self::create_permission))
        .route("/{id}", web::get().to(Self::get_permission))
        .route("/{id}", web::put().to(Self::update_permission))
        .route("/{id}", web::delete().to(Self::delete_permission)),
    );
  }

  async fn create_permission(req: web::Json<CreatePermissionRequest>, pool: web::Data<PgPool>) -> impl Responder {
    info!("Processing create permission request for name: {}", req.name);
    if let Err(e) = req.validate() {
      error!("Validation failed for permission creation: {}", e);
      return HttpResponse::BadRequest().json(serde_json::json!({
        "error": format!("Validation error: {}", e)
      }));
    }
    let handler = PermissionHandler::new(&pool);
    match handler.create(&req.name, req.description.as_deref()) {
      Ok(permission) => {
        info!("Permission created successfully via route: {}", permission.name);
        HttpResponse::Ok().json(PermissionResponse::from(permission))
      }
      Err(e) => {
        error!("Failed to create permission {}: {}", req.name, e);
        e.error_response()
      }
    }
  }

  async fn get_permission(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing get permission request for ID: {}", id);
    let handler = PermissionHandler::new(&pool);
    match handler.find_by_id(id) {
      Ok(permission) => {
        info!("Permission retrieved successfully: {}", id);
        HttpResponse::Ok().json(PermissionResponse::from(permission))
      }
      Err(e) => {
        error!("Failed to retrieve permission {}: {}", id, e);
        e.error_response()
      }
    }
  }

  async fn update_permission(path: web::Path<Uuid>, req: web::Json<UpdatePermissionRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing update permission request for ID: {}", id);
    if let Err(e) = req.validate() {
      error!("Validation failed for permission update: {}", e);
      return HttpResponse::BadRequest().json(serde_json::json!({
        "error": format!("Validation error: {}", e)
      }));
    }
    let handler = PermissionHandler::new(&pool);
    match handler.update(id, req.name.as_deref(), req.description.as_deref()) {
      Ok(permission) => {
        info!("Permission updated successfully: {}", id);
        HttpResponse::Ok().json(PermissionResponse::from(permission))
      }
      Err(e) => {
        error!("Failed to update permission {}: {}", id, e);
        e.error_response()
      }
    }
  }

  async fn delete_permission(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing delete permission request for ID: {}", id);
    let handler = PermissionHandler::new(&pool);
    match handler.delete(id) {
      Ok(()) => {
        info!("Permission deleted successfully: {}", id);
        HttpResponse::Ok().finish()
      }
      Err(e) => {
        error!("Failed to delete permission {}: {}", id, e);
        e.error_response()
      }
    }
  }
}