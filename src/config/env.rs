

use std::sync::LazyLock;
// use serde::Deserialize;
// #[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Env {
    pub ENVIRONMENT: String,
    pub APP_VERSION: String,
    pub APP_PORT: u16,

    pub ALLOWED_ORIGINS: Vec<String>,

    pub POSTGRES_HOST: String,
    pub POSTGRES_PORT: u16,
    pub POSTGRES_DB: String,
    pub POSTGRES_USER: String,
    pub POSTGRES_PASSWORD: String,

    pub RUST_LOG: String,
    pub TZ: String
}

impl Env {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Self {
            ENVIRONMENT: std::env::var("ENVIRONMENT").expect("ENVIRONMENT missing in .env"),
            APP_VERSION: std::env::var("APP_VERSION").expect("APP_VERSION missing in .env"),
            APP_PORT: std::env::var("APP_PORT").expect("PORT missing in .env").parse::<u16>().expect("PORT is not Correct Format"),
            ALLOWED_ORIGINS: std::env::var("ALLOWED_ORIGINS").expect("ALLOWED_ORIGINS missing in .env")
                                .split(',')
                                .map(|url| url.trim().to_string())
                                .collect(),
            POSTGRES_HOST: std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST missing in .env"),
            POSTGRES_PORT: std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT missing in .env").parse::<u16>().expect("POSTGRES_PORT is not Correct Format"),
            POSTGRES_DB: std::env::var("POSTGRES_DB").expect("POSTGRES_DB missing in .env"),
            POSTGRES_USER: std::env::var("POSTGRES_USER").expect("POSTGRES_USER missing in .env"),
            POSTGRES_PASSWORD: std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD missing in .env"),
            RUST_LOG: std::env::var("RUST_LOG").expect("RUST_LOG missing in .env"),
            TZ: std::env::var("TZ").expect("TZ missing in .env")


        }
    }


}



pub static ENV: LazyLock<Env> = LazyLock::new(Env::new);