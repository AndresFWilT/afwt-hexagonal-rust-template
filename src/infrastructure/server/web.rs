use actix_cors::Cors;
use actix_web::http;

pub fn setup_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec![http::Method::GET, http::Method::OPTIONS])
        .allowed_headers(vec![http::header::ACCESS_CONTROL_ALLOW_HEADERS, http::header::ORIGIN, http::header::ACCEPT, http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .max_age(3600)
}
