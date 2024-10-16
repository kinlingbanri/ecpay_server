use actix_web::{post, HttpResponse, Responder};
use log::info;

#[post("/OtherRoute")]
async fn other_route() -> impl Responder {
    info!("OtherRoute route called");
    HttpResponse::Ok().body("OtherRoute response")
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(other_route);
}
