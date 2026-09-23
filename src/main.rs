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
use crate::modules::routes::{open_routes, protected_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = create_pool();

    migrations::run(&pool)
        .await
        .expect("Database Migration Failed");

    HttpServer::new( move || {
        App::new()
            .wrap(configure_cors())
            .app_data(web::Data::new(pool.clone()))
            .configure(open_routes)
            .configure(protected_routes)
    })
        .bind(("0.0.0.0", ENV.APP_PORT))?
        .run()
        .await
}
