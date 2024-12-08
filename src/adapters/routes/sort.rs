use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/v1/sort/bubble")
            .route(web::get().to(crate::adapters::controllers::sort::bubble_controller))
    );
}