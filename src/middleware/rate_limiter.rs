use std::{
    rc::Rc
};
use futures_util::future::{ready, LocalBoxFuture, Ready};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use actix_web::web::Data;
use redis::aio::ConnectionManager;
use redis::AsyncTypedCommands;
use crate::utils::error::AppError;
use crate::utils::token::Claims;

#[derive(Clone, Copy)]
pub enum RateLimitKey {
    Ip,
    UserId,
}

pub struct RateLimitMiddleware {
    pub max_requests: u64,
    pub window_sec: u64,
    key_type: RateLimitKey
}

impl RateLimitMiddleware {
    pub fn ip(max_requests: u64, window_sec: u64) -> Self {
        Self {
            max_requests,
            window_sec,
            key_type: RateLimitKey::Ip
        }
    }

    pub fn user(max_requests: u64, window_sec: u64) -> Self {
        Self {
            max_requests,
            window_sec ,
            key_type: RateLimitKey::UserId
        }
    }

}


impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static {

    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimitService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(
            Ok(
                RateLimitService {
                    service: Rc::new(service),
                    max_requests: self.max_requests,
                    window_sec: self.window_sec,
                    key_type: self.key_type,
                }
            )
        )
    }



}


pub struct RateLimitService<S> {
    service: Rc<S>,
    max_requests: u64,
    window_sec: u64,
    key_type: RateLimitKey
}

impl<S, B> Service<ServiceRequest> for RateLimitService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static {

    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);


    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let max_requests = self.max_requests;
        let window_sec = self.window_sec;
        let key_type = self.key_type;

        Box::pin( async move {
            let mut redis = req
                .app_data::<Data<ConnectionManager>>()
                .cloned()
                .ok_or_else( || {AppError::InternalServerError})?
                .get_ref()
                .clone();

            let key = match key_type {
                RateLimitKey::Ip => {
                    let connection_info = req.connection_info();
                    let ip = connection_info
                        .realip_remote_addr()
                        .unwrap_or("UNKNOWN");

                    format!("USER:IP:{ip}")
                }

                RateLimitKey::UserId => {
                    let extensions = req.extensions();
                    let claims = extensions
                        .get::<Claims>()
                        .ok_or_else(|| {
                            AppError::Unauthorized(
                                "Authentication required".into()
                            )
                        })?;

                    format!("USER:ID:{}", claims.sub)
                }
            };


            let count = redis.
                incr(&key, 1)
                .await
                .map_err(|_| {
                AppError::InternalServerError
            })?;

            // First request starts the window.
            if count == 1 {

                redis.expire(&key, window_sec as i64).await
                    .map_err(|_| {
                        AppError::InternalServerError
                    })?;
            }

            if count > max_requests as isize {

                return Err(
                    AppError::TooManyRequests(
                        "Too many requests".into()
                    )
                        .into()
                );
            }

            service.call(req).await


        })
    }












}