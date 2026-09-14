

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use crate::config::env::ENV;
use crate::utils::error::AppError;

#[derive(Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    // pub token_type: String,
}

pub fn create_access_token(user_id: i64) -> Result<String, AppError> {
    let now = chrono::Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 15 * 60
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            ENV.JWT_ACCESS_SECRET
                .as_bytes(),
        ),
    )
        .map_err(|_| AppError::InternalServerError)
}