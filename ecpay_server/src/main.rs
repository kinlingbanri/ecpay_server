mod db;
mod ecpay_api;
mod logger;
mod models;
mod repositories;
mod routes;
mod services;

use actix_web::{App, HttpServer};
use db::init_db_pool;
use log::{error, info};
use repositories::account_repository::AccountRepository;
use reqwest::Client;
use services::account_service::AccountService;

#[allow(unused)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    logger::init();
    info!("logger init finished");

    // init db
    let db_pool = init_db_pool().await;

    // 創建 AccountRepository 和 AccountService
    let account_repository = AccountRepository {
        pool: db_pool.clone(),
    };
    let account_service = AccountService {
        repository: account_repository,
    };

    // 查詢所有帳號
    match account_service.get_all_accounts().await {
        Ok(accounts) => {
            for account in accounts {
                info!("Account: {:?}", account);
            }
        }
        Err(e) => error!("Error: {}", e),
    }

    let client = Client::new();
    // 啟動 HTTP, WebSocket 伺服器
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(client.clone()))
            .configure(routes::ecpay::config) // 配置 ECPay 路由
            .configure(routes::neweb::config) // 配置 Neweb 路由
            .configure(routes::other::config) // 配置其他路由
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
