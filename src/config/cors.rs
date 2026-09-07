use actix_cors::Cors;
use actix_web::http::header;
use crate::config::env::ENV;

pub fn configure_cors() -> Cors {

    let origins = ENV.ALLOWED_ORIGINS.clone();

    Cors::default()
        .allowed_origin_fn( move |origin, _req_head| {
            origins
                .iter()
                .any(|allowed| allowed.as_bytes() == origin.as_bytes())
        })
        .allow_any_method()
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ])
        .supports_credentials()
        .max_age(3600)

}