use actix_web::{web, HttpResponse, Responder, ResponseError};
use crate::handlers::auth::{AuthHandler, RegisterRequest, LoginRequest};

pub struct AuthRoutes;

impl AuthRoutes {
  pub fn configure(cfg: &mut web::ServiceConfig) {
      cfg.service(
          web::scope("/auth")
              .route("/register", web::post().to(Self::register))
              .route("/login", web::post().to(Self::login))
      );
  }

  async fn register(
      pool: web::Data<crate::database::PgPool>,
      // config: web::Data<crate::config::Config>,
      req: web::Json<RegisterRequest>,
  ) -> impl Responder {
      let auth_handler = AuthHandler::new(&pool);
      match auth_handler.register(&req) {
          Ok(user_response) => HttpResponse::Ok().json(user_response),
          Err(e) => e.error_response(),
      }
  }

  async fn login(
      pool: web::Data<crate::database::PgPool>,
      // config: web::Data<crate::config::Config>,
      req: web::Json<LoginRequest>,
  ) -> impl Responder {
      let auth_handler = AuthHandler::new(&pool);
      match auth_handler.login(&req) {
          Ok(login_response) => HttpResponse::Ok().json(login_response),
          Err(e) => e.error_response(),
      }
  }
}