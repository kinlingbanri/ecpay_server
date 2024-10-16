use std::collections::HashMap;

use crate::ecpay_api;
use actix_web::{post, web, HttpResponse, Responder};
use log::{error, info};

#[post("/EcpayResult")]
async fn ecpay_result(form: web::Form<HashMap<String, String>>) -> impl Responder {
    info!("EcpayResult route called");

    let hash_key = "pwFHCqoQZGmho4w6";
    let hash_iv = "EkRm7iFT261dpevs";
    let input = form.into_inner();

    // 模擬呼叫 Ecpay API 進行驗證
    match ecpay_api::is_check_mac_value_match(hash_key, hash_iv, &input) {
        Ok(ecpay_result) => {
            if ecpay_result.is_match > 0 {
                HttpResponse::Ok().body("1|OK")
            } else {
                HttpResponse::BadRequest().body("MAC 值匹配失敗")
            }
        }
        Err(e) => {
            error!("MAC 值驗證失敗: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(ecpay_result);
}
