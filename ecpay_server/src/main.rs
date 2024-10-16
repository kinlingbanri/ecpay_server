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
use reqwest::Client;

use repositories::account_repository::AccountRepository;
use repositories::admin_config_respsitory::AdminConfigRepository;
use repositories::cluster_repository::ClusterRepository;

use services::account_service::AccountService;
use services::admin_config_service::AdminConfigService;
use services::cluster_service::ClusterService;

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

    let admin_config_repository = AdminConfigRepository {
        pool: db_pool.clone(),
    };
    let admin_config_service = AdminConfigService::new(admin_config_repository);

    let cluster_repository = ClusterRepository {
        pool: db_pool.clone(), // 使用正確的花括號初始化結構體字段
    };
    let cluster_service = ClusterService::new(cluster_repository);

    // 查詢所有帳號
    match account_service.get_all_accounts().await {
        Ok(accounts) => {
            for account in accounts {
                info!("Account: {:?}", account);
            }
        }
        Err(e) => error!("Error: {}", e),
    }

    match admin_config_service.get_admin_config_by_id(1).await {
        Ok(Some(admin_config)) => {
            // 將結果存入實例變量
            let admin_config_model = admin_config;

            // 使用 println! 打印結果
            //println!("Found AdminConfig: {:?}", admin_config_model);

            // 分別顯示各個欄位的值
            println!("ID: {}", admin_config_model.id);
            println!("OCPP FTP IP: {:?}", admin_config_model.ocpp_ftp_ip); // 使用 {:?} 因為可能是 Option
            println!("OCPP FTP Port: {}", admin_config_model.ocpp_ftp_port);
            println!("OCPP FTP Path: {:?}", admin_config_model.ocpp_ftp_path); // 使用 {:?} 因為可能是 Option
            println!("OCPP FTP User: {:?}", admin_config_model.ocpp_ftp_user); // 使用 {:?} 因為可能是 Option
            println!("OCPP FTP Password: {:?}", admin_config_model.ocpp_ftp_pwd); // 使用 {:?} 因為可能是 Option
            println!("IP: {:?}", admin_config_model.ip); // 使用 {:?} 因為可能是 Option
            println!("Port: {}", admin_config_model.port);
            println!("Backup Disk Path: {}", admin_config_model.backup_disk_path);
            println!("Product: {}", admin_config_model.product);
            println!("OCPP Port: {}", admin_config_model.ocpp_port);
            println!("Is Enc Key: {:?}", admin_config_model.is_enc_key); // 使用 {:?} 因為可能是 Option
        }
        Ok(None) => {
            println!("AdminConfig not found");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    // 測試：根據 ID 獲取 Cluster 資料
    let cid = 1;
    match cluster_service.get_cluster_by_id(cid).await {
        Ok(Some(cluster)) => {
            println!("Cluster: {:?}", cluster);
        }
        Ok(None) => {
            println!("No cluster found with cid {}", cid);
        }
        Err(e) => {
            eprintln!("Error fetching cluster: {:?}", e);
        }
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
