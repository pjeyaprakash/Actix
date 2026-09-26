use redis::aio::ConnectionManager;
use redis::AsyncTypedCommands;
use crate::utils::constants::{LOGIN_MAX_ATTEMPT, LOGIN_WINDOW_SEC};
use crate::utils::error::AppError;

pub async fn email_rate_limiter(redis_conn: &ConnectionManager, user_id: &str) -> Result<(), AppError> {
    let key = format!("USER:{user_id}");
    let mut redis = redis_conn.clone();

    let count = redis
        .incr(&key, 1)
        .await?;

    if count == 1 {
        redis
            .expire(&key, LOGIN_WINDOW_SEC as i64)
            .await?;
    }

    if count > LOGIN_MAX_ATTEMPT as isize {
        return Err(
            AppError::TooManyRequests("Too many login attempts".into())
        )
    }

    Ok(())
}