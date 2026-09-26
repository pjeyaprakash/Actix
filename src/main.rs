mod config;
mod utils;
pub mod proto;
pub mod modules;
pub mod middleware;

use actix_web::{web, App, HttpServer};
use crate::config::{
    migrations,
    db::create_pool,
    cors::configure_cors,
    env::ENV
};
use crate::config::redis::create_redis_manager;
use crate::modules::routes::{public_routes, protected_routes, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = create_pool();
    migrations::run(&pool)
        .await
        .expect("Database Migration Failed");

    let redis = create_redis_manager().await.expect("Failed to Connect Redis");

    HttpServer::new( move || {
        App::new()
            .wrap(configure_cors())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .configure(routes)
    })
        .bind(("0.0.0.0", ENV.APP_PORT))?
        .run()
        .await
}
