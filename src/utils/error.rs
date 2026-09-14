use actix_web::{
    HttpResponse,
    ResponseError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] deadpool_postgres::PoolError),

    #[error("PostgreSQL error")]
    Postgres(#[from] tokio_postgres::Error),

    #[error("Not found")]
    NotFound,

    #[error("Invalid request")]
    BadRequest,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound => {
                HttpResponse::NotFound().finish()
            }

            Self::BadRequest => {
                HttpResponse::BadRequest().finish()
            }

            Self::Database(_) |
            Self::Postgres(_) => {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}