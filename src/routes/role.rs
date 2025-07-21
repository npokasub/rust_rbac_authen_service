use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use crate::database::PgPool;
use crate::handlers::role::RoleHandler;

#[derive(Deserialize)]
pub struct CreateRoleRequest {
  pub name: String,
  pub description: Option<String>,
}

pub struct RoleRoutes;

impl RoleRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/roles")
        .service(web::resource("").route(web::post().to(Self::create_role)))
        .service(web::resource("/{id}").route(web::get().to(Self::get_role)))
    );
  }

  async fn create_role(req: web::Json<CreateRoleRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let role_handler = RoleHandler::new(&pool);
    match role_handler.create(&req.name, req.description.as_deref()) {
      Ok(role) => HttpResponse::Ok().json(serde_json::json!({ "role": role })),
      Err(e) => e.error_response(),
    }
  }

  async fn get_role(path: web::Path<uuid::Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let role_handler = RoleHandler::new(&pool);
    match role_handler.find_by_id(*path) {
      Ok(role) => HttpResponse::Ok().json(serde_json::json!({ "role": role })),
      Err(e) => e.error_response(),
    }
  }
}