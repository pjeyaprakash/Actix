mod config;
mod utils;

pub mod proto;
pub mod modules;

use actix_web::{web, App, HttpServer};
use crate::config::{
    migrations,
    db::create_pool,
    cors::configure_cors,
    env::ENV
};
use crate::modules::user::repo::UserRepo;

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
    })
        .bind(("0.0.0.0", ENV.APP_PORT))?
        .run()
        .await
}
