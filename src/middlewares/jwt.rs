use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures::future::{self, LocalBoxFuture, Ready};
use crate::utilities::error::AppError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

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
    let auth_header = match req.headers().get("Authorization") {
      Some(header) => match header.to_str() {
        Ok(h) => h,
        Err(_) => return Box::pin(future::err(AppError::Unauthorized("Invalid Authorization header".into()).into())),
      },
      None => return Box::pin(future::err(AppError::Unauthorized("Missing Authorization header".into()).into())),
    };

    let token = match auth_header.strip_prefix("Bearer ") {
      Some(token) => token,
      None => return Box::pin(future::err(AppError::Unauthorized("Invalid Authorization header".into()).into())),
    };

    let config = match req.app_data::<actix_web::web::Data<crate::config::Config>>() {
      Some(config) => config,
      None => return Box::pin(future::err(AppError::InternalError.into())),
    };

    let validation = Validation::default();
    let token_data = match decode::<Claims>(
      token,
      &DecodingKey::from_secret(config.auth.jwt_secret.as_ref()),
      &validation,
    ) {
      Ok(data) => data,
      Err(_) => return Box::pin(future::err(AppError::Unauthorized("Invalid or expired token".into()).into())),
    };

    let user_id = match uuid::Uuid::parse_str(&token_data.claims.sub) {
      Ok(id) => id,
      Err(_) => return Box::pin(future::err(AppError::Unauthorized("Invalid user ID in token".into()).into())),
    };

    req.extensions_mut().insert(token_data.claims);
    let service = Rc::clone(&self.service);
    Box::pin(async move { service.call(req).await })
  }
}