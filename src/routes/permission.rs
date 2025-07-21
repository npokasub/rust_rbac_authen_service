use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use crate::database::PgPool;
use crate::handlers::permission::PermissionHandler;

#[derive(Deserialize)]
pub struct CreatePermissionRequest {
  pub name: String,
  pub description: Option<String>,
}

pub struct PermissionRoutes;

impl PermissionRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/permissions")
        .service(web::resource("").route(web::post().to(Self::create_permission)))
        .service(web::resource("/{id}").route(web::get().to(Self::get_permission)))
    );
  }

  async fn create_permission(req: web::Json<CreatePermissionRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let permission_handler = PermissionHandler::new(&pool);
    match permission_handler.create(&req.name, req.description.as_deref()) {
      Ok(permission) => HttpResponse::Ok().json(serde_json::json!({ "permission": permission })),
      Err(e) => e.error_response(),
    }
  }

  async fn get_permission(path: web::Path<uuid::Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let permission_handler = PermissionHandler::new(&pool);
    match permission_handler.find_by_id(*path) {
      Ok(permission) => HttpResponse::Ok().json(serde_json::json!({ "permission": permission })),
      Err(e) => e.error_response(),
    }
  }
}