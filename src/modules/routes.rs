use actix_web::web::{
    scope,
    ServiceConfig
};
use crate::middleware::authentication::AuthMiddleware;
use crate::middleware::rate_limiter::RateLimitMiddleware;
use crate::modules::auth::route::auth_route;
use crate::modules::user::routes::user_routes;

pub fn public_routes(service_config: &mut ServiceConfig) {
    service_config
        .service(scope("/auth").configure(auth_route));
}


pub fn protected_routes(service_config: &mut ServiceConfig) {
    service_config
        .service(scope("/user").configure(user_routes));
}


pub fn routes(service_config: &mut ServiceConfig) {
    service_config
        .service(
            scope("/pub")
                .wrap(RateLimitMiddleware::ip(5, 100))
                .configure(public_routes)
        )
        .service(
            scope("/api")
                .wrap(RateLimitMiddleware::user(100, 60))
                .wrap(AuthMiddleware)
                .configure(protected_routes)
        );
}