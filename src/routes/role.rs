use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use validator::Validate;
use crate::database::PgPool;
use crate::handlers::role::RoleHandler;
use crate::models::role::RoleResponse;
use log::{error, info};
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct CreateRoleRequest {
  #[validate(length(min = 2))]
  pub name: String,
  #[validate(length(min = 1))]
  pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateRoleRequest {
  #[validate(length(min = 2))]
  pub name: Option<String>,
  #[validate(length(min = 1))]
  pub description: Option<String>,
}

pub struct RoleRoutes;

impl RoleRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/roles")
        .wrap(crate::middlewares::jwt::JwtMiddleware)
        .route("", web::post().to(Self::create_role))
        .route("/{id}", web::get().to(Self::get_role))
        .route("/{id}", web::put().to(Self::update_role))
        .route("/{id}", web::delete().to(Self::delete_role)),
    );
  }

  async fn create_role(req: web::Json<CreateRoleRequest>, pool: web::Data<PgPool>) -> impl Responder {
    info!("Processing create role request for name: {}", req.name);
    if let Err(e) = req.validate() {
      error!("Validation failed for role creation: {}", e);
      return HttpResponse::BadRequest().json(serde_json::json!({
        "error": format!("Validation error: {}", e)
      }));
    }
    let handler = RoleHandler::new(&pool);
    match handler.create(&req.name, req.description.as_deref()) {
      Ok(role) => {
        info!("Role created successfully via route: {}", role.name);
        HttpResponse::Ok().json(RoleResponse::from(role))
      }
      Err(e) => {
        error!("Failed to create role {}: {}", req.name, e);
        e.error_response()
      }
    }
  }

  async fn get_role(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing get role request for ID: {}", id);
    let handler = RoleHandler::new(&pool);
    match handler.find_by_id(id) {
      Ok(role) => {
        info!("Role retrieved successfully: {}", id);
        HttpResponse::Ok().json(RoleResponse::from(role))
      }
      Err(e) => {
        error!("Failed to retrieve role {}: {}", id, e);
        e.error_response()
      }
    }
  }

  async fn update_role(path: web::Path<Uuid>, req: web::Json<UpdateRoleRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing update role request for ID: {}", id);
    if let Err(e) = req.validate() {
      error!("Validation failed for role update: {}", e);
      return HttpResponse::BadRequest().json(serde_json::json!({
        "error": format!("Validation error: {}", e)
      }));
    }
    let handler = RoleHandler::new(&pool);
    match handler.update(id, req.name.as_deref(), req.description.as_deref()) {
      Ok(role) => {
        info!("Role updated successfully: {}", id);
        HttpResponse::Ok().json(RoleResponse::from(role))
      }
      Err(e) => {
        error!("Failed to update role {}: {}", id, e);
        e.error_response()
      }
    }
  }

  async fn delete_role(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let id = *path;
    info!("Processing delete role request for ID: {}", id);
    let handler = RoleHandler::new(&pool);
    match handler.delete(id) {
      Ok(()) => {
        info!("Role deleted successfully: {}", id);
        HttpResponse::Ok().finish()
      }
      Err(e) => {
        error!("Failed to delete role {}: {}", id, e);
        e.error_response()
      }
    }
  }
}