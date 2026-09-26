use redis::aio::ConnectionManager;
use redis::RedisError;
use crate::config::env::ENV;

pub async fn create_redis_manager() -> Result<ConnectionManager, RedisError> {
    let client = redis::Client::open(ENV.REDIS_URL.clone());

    let manager = client.unwrap().get_connection_manager().await?;

    Ok(manager)
}