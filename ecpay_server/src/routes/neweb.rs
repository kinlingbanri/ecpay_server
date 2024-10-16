use actix_web::{post, HttpResponse, Responder};
use log::info;

#[post("/NewebNotify")]
async fn neweb_notify() -> impl Responder {
    info!("NewebNotify route called");
    HttpResponse::Ok().body("NewebNotify received")
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(neweb_notify);
}
