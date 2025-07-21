use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use crate::database::PgPool;
use crate::handlers::role_permission::RolePermissionHandler;

#[derive(Deserialize)]
pub struct AssignPermissionRequest {
  pub role_id: uuid::Uuid,
  pub permission_id: uuid::Uuid,
}

pub struct RolePermissionRoutes;

impl RolePermissionRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/role_permissions")
        .service(web::resource("").route(web::post().to(Self::assign_permission)))
        .service(web::resource("/{role_id}").route(web::get().to(Self::get_role_permissions)))
    );
  }

  async fn assign_permission(req: web::Json<AssignPermissionRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let role_permission_handler = RolePermissionHandler::new(&pool);
    match role_permission_handler.assign_permission(req.role_id, req.permission_id) {
      Ok(role_permission) => HttpResponse::Ok().json(serde_json::json!({ "role_permission": role_permission })),
      Err(e) => e.error_response(),
    }
  }

  async fn get_role_permissions(path: web::Path<uuid::Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let role_permission_handler = RolePermissionHandler::new(&pool);
    match role_permission_handler.find_by_role_id(*path) {
      Ok(role_permissions) => HttpResponse::Ok().json(serde_json::json!({ "role_permissions": role_permissions })),
      Err(e) => e.error_response(),
    }
  }
}