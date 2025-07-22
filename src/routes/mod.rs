use actix_web::web;

pub mod auth;
pub mod user;
pub mod role;
pub mod permission;
pub mod user_role;
pub mod role_permission;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/api")
      .configure(auth::AuthRoutes::configure)
      .service(
        web::scope("")
          .wrap(crate::middlewares::jwt::JwtMiddleware)
          .configure(user::UserRoutes::configure)
          .configure(role::RoleRoutes::configure)
          .configure(permission::PermissionRoutes::configure)
          .configure(user_role::UserRoleRoutes::configure)
          .configure(role_permission::RolePermissionRoutes::configure)
      )
  );
}