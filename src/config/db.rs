use deadpool_postgres::{
    Manager,
    ManagerConfig,
    Pool,
    RecyclingMethod,
};
use tokio_postgres::{Config, NoTls};
use crate::config::env::ENV;
use crate::utils::constants::DB_MAX_CONNECTIONS;

pub fn create_pool() -> Pool {
    let mut pg_config = Config::new();

    pg_config.host(ENV.POSTGRES_HOST.clone());
    pg_config.port(ENV.POSTGRES_PORT);
    pg_config.dbname(ENV.POSTGRES_DB.clone());
    pg_config.user(ENV.POSTGRES_USER.clone());
    pg_config.password(ENV.POSTGRES_PASSWORD.clone());

    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let manager = Manager::from_config(
        pg_config,
        NoTls,
        manager_config
    );

    Pool::builder(manager)
        .max_size(DB_MAX_CONNECTIONS)
        .build()
        .expect("Failed to create PostgreSQL pool")
}