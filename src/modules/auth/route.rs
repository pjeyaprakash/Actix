use actix_web::{
    web,
    web::ServiceConfig
};
use crate::modules::auth::handler::AuthHandler;

pub fn auth_route(service_config: &mut ServiceConfig) {
    service_config
        .route("/login", web::post().to(AuthHandler::login));
}