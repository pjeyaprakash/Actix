use actix_web::{
    web,
    web::ServiceConfig
};
use crate::modules::user::handler::UserHandler;

pub fn user_routes(service_config: &mut ServiceConfig) {
    service_config
        .route("/get", web::get().to(UserHandler::get));
}