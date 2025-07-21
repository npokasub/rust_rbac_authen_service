use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use crate::database::PgPool;
use crate::handlers::user_role::UserRoleHandler;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
  pub user_id: uuid::Uuid,
  pub role_id: uuid::Uuid,
}

pub struct UserRoleRoutes;

impl UserRoleRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/user_roles")
        .service(web::resource("").route(web::post().to(Self::assign_role)))
        .service(web::resource("/{user_id}").route(web::get().to(Self::get_user_roles)))
    );
  }

  async fn assign_role(req: web::Json<AssignRoleRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let user_role_handler = UserRoleHandler::new(&pool);
    match user_role_handler.assign_role(req.user_id, req.role_id) {
      Ok(user_role) => HttpResponse::Ok().json(serde_json::json!({ "user_role": user_role })),
      Err(e) => e.error_response(),
    }
  }

  async fn get_user_roles(path: web::Path<uuid::Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let user_role_handler = UserRoleHandler::new(&pool);
    match user_role_handler.find_by_user_id(*path) {
      Ok(user_roles) => HttpResponse::Ok().json(serde_json::json!({ "user_roles": user_roles })),
      Err(e) => e.error_response(),
    }
  }
}