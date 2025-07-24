use actix_web::{web, dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures::future::{self, LocalBoxFuture, Ready};
use crate::database::PgPool;
use crate::utilities::error::AppError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use log::{info, debug, error};
use crate::schema::{user_roles, roles, role_permissions, permissions};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize,
}

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = JwtMiddlewareService<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    debug!("Initializing JwtMiddleware");
    future::ok(JwtMiddlewareService { service: Rc::new(service) })
  }
}

pub struct JwtMiddlewareService<S> {
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  actix_web::dev::forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    info!("Processing request: {}", req.path());
    let pool = match req.app_data::<web::Data<PgPool>>() {
      Some(pool) => pool.clone(),
      None => {
        error!("Database pool not found in request");
        return Box::pin(future::err(AppError::ConnectionError("Database pool not found".into()).into()));
      }
    };

    let config = match req.app_data::<actix_web::web::Data<crate::config::Config>>() {
      Some(config) => config,
      None => {
        error!("Failed to access config");
        return Box::pin(future::err(AppError::BadRequest("Failed to access configuration".into()).into()));
      }
    };

    let auth_header = match req.headers().get("Authorization") {
      Some(header) => match header.to_str() {
        Ok(h) => {
          debug!("Authorization header found: {}", h);
          h
        },
        Err(e) => {
          error!("Invalid Authorization header: {}", e);
          return Box::pin(future::err(AppError::Unauthorized("Invalid Authorization header".into()).into()));
        },
      },
      None => {
        error!("Missing Authorization header");
        return Box::pin(future::err(AppError::Unauthorized("Missing Authorization header".into()).into()));
      },
    };

    let token = match auth_header.strip_prefix("Bearer ") {
      Some(token) => {
        debug!("Bearer token extracted");
        token
      },
      None => {
        error!("Invalid Authorization header: no Bearer prefix");
        return Box::pin(future::err(AppError::Unauthorized("Invalid Authorization header".into()).into()));
      },
    };

    let validation = Validation::default();
    let token_data = match decode::<Claims>(
      token,
      &DecodingKey::from_secret(config.auth.jwt_secret.as_ref()),
      &validation,
    ) {
      Ok(data) => {
        debug!("Token decoded successfully, user_id: {}", data.claims.sub);
        data
      },
      Err(e) => {
        error!("Token validation failed: {}", e);
        return Box::pin(future::err(AppError::Unauthorized("Invalid or expired token".into()).into()));
      },
    };

    let user_id = match Uuid::parse_str(&token_data.claims.sub) {
      Ok(id) => {
        debug!("Valid user_id in token: {}", id);
        id
      },
      Err(e) => {
        error!("Invalid user_id in token: {}", e);
        return Box::pin(future::err(AppError::Unauthorized("Invalid user ID in token".into()).into()));
      },
    };

    let has_permission = match check_user_permission(&pool, user_id, &req) {
      Ok(_) => true,
      Err(e) => {
        error!("Permission check failed for user_id={} on {}: {}", user_id, req.path(), e);
        return Box::pin(future::err(e.into()));
      }
    };

    if !has_permission {
      error!("User {} does not have required permission for {}", user_id, req.path());
      return Box::pin(future::err(AppError::Forbidden.into()));
    }

    info!("Token validated for user_id: {}", user_id);
    req.extensions_mut().insert(token_data.claims);
    let path = req.path().to_string();
    let service = Rc::clone(&self.service);
    Box::pin(async move {
      let response = service.call(req).await;
      match &response {
        Ok(_) => info!("Request processed successfully for path: {}", path),
        Err(e) => error!("Request failed for path: {}: {}", path, e),
      }
      response
    })
  }
}

fn check_user_permission(pool: &PgPool, user_id: Uuid, req: &ServiceRequest) -> Result<(), AppError> {
  let required_permission = match req.path() {
    path if path.starts_with("/api/user_roles") => match req.method().as_str() {
      "POST" => Some("admin.create_user_role"),
      "DELETE" => Some("admin.delete_user_role"),
      _ => None,
    },
    // Add other routes here as needed
    _ => None,
  };

  if let Some(permission_name) = required_permission {
    debug!("Checking permission {} for user_id={}", permission_name, user_id);
    let mut conn = pool.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    let has_permission = user_roles::table
      .inner_join(roles::table)
      .inner_join(role_permissions::table.on(role_permissions::role_id.eq(roles::id)))
      .inner_join(permissions::table.on(permissions::id.eq(role_permissions::permission_id)))
      .filter(user_roles::user_id.eq(user_id))
      .filter(permissions::name.eq(permission_name))
      .select(permissions::name)
      .first::<String>(&mut conn)
      .optional()
      .map_err(|e| {
        error!("Failed to check permission {} for user_id={}: {:?}", permission_name, user_id, e);
        AppError::from(e)
      })?
      .is_some();

    if has_permission {
      info!("User {} has permission {}", user_id, permission_name);
      Ok(())
    } else {
      error!("User {} lacks permission {}", user_id, permission_name);
      Err(AppError::Forbidden)
    }
  } else {
    debug!("No specific permission required for {}", req.path());
    Ok(())
  }
}