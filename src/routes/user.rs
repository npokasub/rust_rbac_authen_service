use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use crate::database::PgPool;
use crate::handlers::user::UserHandler;

#[derive(Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

pub struct UserRoutes;

impl UserRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/users")
        .service(web::resource("").route(web::post().to(Self::create_user)))
        .service(web::resource("/{id}").route(web::get().to(Self::get_user)))
    );
  }

  async fn create_user(req: web::Json<CreateUserRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let user_handler = UserHandler::new(&pool);
    match user_handler.create(&req.username, &req.email, &req.password) {
      Ok(user) => HttpResponse::Ok().json(serde_json::json!({ "user": user })),
      Err(e) => e.error_response(),
    }
  }

  async fn get_user(path: web::Path<uuid::Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let user_handler = UserHandler::new(&pool);
    match user_handler.find_by_id(*path) {
      Ok(user) => HttpResponse::Ok().json(serde_json::json!({ "user": user })),
      Err(e) => e.error_response(),
    }
  }
}