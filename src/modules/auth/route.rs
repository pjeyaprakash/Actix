use actix_web::web::{ServiceConfig, post};
use crate::modules::auth::handler::AuthHandler;

pub fn auth_route(service_config: &mut ServiceConfig) {
    service_config
        .route("/login", post().to(AuthHandler::login))
        .route("/signup", post().to(AuthHandler::signup));
}