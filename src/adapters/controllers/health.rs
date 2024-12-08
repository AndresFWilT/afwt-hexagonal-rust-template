use actix_web::{HttpResponse, Responder};

pub async fn health_controller() -> impl Responder {
    HttpResponse::Ok().json("Server healthy")
}
