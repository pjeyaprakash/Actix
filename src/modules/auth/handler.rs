use actix_web::{web, HttpResponse};
use bytes::Bytes;
use deadpool_postgres::Pool;
use prost::Message;
use crate::modules::auth::service::AuthService;
use crate::proto::auth::LoginRequest;
use crate::utils::response::protobuf_response;
use crate::utils::error::AppError;

pub struct AuthHandler;

impl AuthHandler {

    pub async fn login(pool: web::Data<Pool>, body: Bytes) -> Result<HttpResponse, AppError> {
        let payload = LoginRequest::decode(body.as_ref())
            .map_err(|_| AppError::BadRequest)?;

        let res = AuthService::login(&pool, payload).await?;
        Ok(protobuf_response(res))

    }
}