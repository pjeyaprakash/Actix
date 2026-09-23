use std::rc::Rc;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures_util::future::{ready, LocalBoxFuture, Ready};
use crate::utils::error::AppError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::config::env::ENV;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;


    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service)
        }))
    }
}


pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}


impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;


    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let token = extract_access_token(&req)
                .ok_or_else( || AppError::Unauthorized("Missing Access Token".into()))?;

            let claims = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(ENV.JWT_ACCESS_SECRET.as_bytes()),
                &Validation::default()
            )
                .map_err(|_| AppError::Unauthorized("Expired Token".into()))?
                .claims;

            service.call(req).await
        })
    }
}


fn extract_access_token(req: &ServiceRequest) -> Option<String> {
    // 1. Bearer token
    if let Some(header) = req.headers().get("Authorization") {
        if let Ok(value) = header.to_str() {
            if let Some(token) = value.strip_prefix("Bearer ") {
                if !token.is_empty() {
                    return Some(token.to_owned());
                }
            }
        }
    }

    // 2. Cookie
    if let Some(cookie) = req.cookie("access_token") {
        return Some(cookie.value().to_owned());
    }

    None
}