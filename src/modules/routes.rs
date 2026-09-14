use actix_web::web::{
    scope,
    ServiceConfig
};
use crate::modules::auth::route::auth_route;
use crate::modules::user::routes::user_routes;

pub fn routes(service_config: &mut ServiceConfig) {
    service_config
        .service(scope("/user").configure(user_routes))
        .service(scope("/auth").configure(auth_route));
}