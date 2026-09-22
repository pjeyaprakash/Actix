use actix_web::{
    http::StatusCode,
    HttpResponse,
    ResponseError,
};
use bcrypt::BcryptError;
use prost::{DecodeError, Message};
use crate::proto::api::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Not found")]
    NotFound,

    #[error("Database error")]
    Database(#[from] deadpool_postgres::PoolError),

    #[error("PostgreSQL error")]
    Postgres(#[from] tokio_postgres::Error),

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Password hashing error")]
    Bcrypt(#[from] BcryptError),

    #[error("Invalid protobuf request")]
    ProtobufDecode(#[from] DecodeError),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::ProtobufDecode(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::NotFound => StatusCode::NOT_FOUND,

            Self::Database(_) |
            Self::Postgres(_) |
            Self::Bcrypt(_) |
            Self::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            Self::BadRequest(message) => {
                ("BAD_REQUEST", message.as_str())
            }

            Self::ProtobufDecode(_) => (
                "BAD_REQUEST",
                "Invalid protobuf request",
            ),

            Self::Unauthorized(message) => {
                ("UNAUTHORIZED", message.as_str())
            }

            Self::Forbidden(message) => {
                ("FORBIDDEN", message.as_str())
            }

            Self::TooManyRequests(message) => {
                ("TOO_MANY_REQUESTS", message.as_str())
            }

            Self::NotFound => {
                ("NOT_FOUND", "Resource not found")
            }

            // Self::Bcrypt(_) => {
            //     ("INTERNAL_SERVER_ERROR", "Internal Server Error")
            // }

            Self::Database(_) |
            Self::Postgres(_) |
            Self::Bcrypt(_) |
            Self::InternalServerError => {
                ("INTERNAL_SERVER_ERROR", "Internal Server Error")
            }
        };

        let response = ErrorResponse {
            code: code.to_string(),
            message: message.to_string(),
        };

        let mut buf = Vec::with_capacity(response.encoded_len());

        response
            .encode(&mut buf)
            .expect("protobuf encoding failed");

        HttpResponse::build(self.status_code())
            .content_type("application/x-protobuf")
            .body(buf)
    }
}










// use actix_web::{
//     HttpResponse,
//     ResponseError,
// };
// use thiserror::Error;
//
// #[derive(Debug, Error)]
// pub enum AppError {
//     #[error("Database error")]
//     Database(#[from] deadpool_postgres::PoolError),
//
//     #[error("PostgreSQL error")]
//     Postgres(#[from] tokio_postgres::Error),
//
//     #[error("Not found")]
//     NotFound,
//
//     #[error("{0}")]
//     BadRequest,
//
//     #[error("Internal Server Error")]
//     InternalServerError,
//     //
//     // #[error("{0}")]
//     // CustomError
// }
//
// impl ResponseError for AppError {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             Self::NotFound => {
//                 HttpResponse::NotFound().finish()
//             }
//
//             // Self::BadRequest => {
//             //     HttpResponse::BadRequest().finish()
//             // }
//
//             Self::InternalServerError => {
//                 HttpResponse::InternalServerError().finish()
//             }
//
//             Self::Database(_) |
//             Self::Postgres(_) => {
//                 HttpResponse::InternalServerError().finish()
//             }
//
//             Self::BadRequest(message) => {
//                 HttpResponse::BadRequest().body(message)
//             }
//         }
//     }
// }