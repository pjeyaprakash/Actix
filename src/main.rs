mod config;
mod utils;

use actix_web::{web, App, HttpServer};
use crate::config::{
    db::create_pool,
    cors::configure_cors,
    env::ENV
};
// use crate::config::;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = create_pool();

    HttpServer::new( move || {
        App::new()
            .wrap(configure_cors())
            .app_data(web::Data::new(pool.clone()))
    })
        .bind(("0.0.0.0", ENV.APP_PORT))?
        .run()
        .await
}
