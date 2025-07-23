use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures::future::{self, LocalBoxFuture, Ready};
use crate::utilities::error::AppError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use log::{info, debug, error};

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
    // debug!("Initializing JwtMiddleware");
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
        return Box::pin(future::err(AppError::Unauthorized("Missing Authorization header".into()).into()))
      },
    };

    let token = match auth_header.strip_prefix("Bearer ") {
      Some(token) => {
        debug!("Bearer token extracted");
        token
      },
      None => {
        error!("Invalid Authorization header: no Bearer prefix");
        return Box::pin(future::err(AppError::Unauthorized("Invalid Authorization header".into()).into()))
      },
    };

    let config = match req.app_data::<actix_web::web::Data<crate::config::Config>>() {
      Some(config) => config,
      None => {
        error!("Failed to access config");
        return Box::pin(future::err(AppError::InternalError.into()))
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
        return Box::pin(future::err(AppError::Unauthorized("Invalid or expired token".into()).into()))
      },
    };

    let user_id = match uuid::Uuid::parse_str(&token_data.claims.sub) {
      Ok(id) => {
        debug!("Valid user_id in token: {}", id);
        id
      },
      Err(e) => {
        error!("Invalid user_id in token: {}", e);
        return Box::pin(future::err(AppError::Unauthorized("Invalid user ID in token".into()).into()))
      },
    };

    info!("Token validated for user_id: {}", user_id);
    req.extensions_mut().insert(token_data.claims);
    let path = req.path().to_string();
    let service = Rc::clone(&self.service);
    Box::pin(async move {
      let response= service.call(req).await;
      match &response {
        Ok(_) => info!("Request processed successfully for path: {}", path),
        Err(e) => error!("Request failed for path: {}: {}", path, e),
      }
      response
    })
  }
}