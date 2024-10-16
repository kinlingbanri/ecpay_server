use async_trait::async_trait;
use bb8::{ManageConnection, Pool};
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv; // 引入 dotenv 用來讀取 .env 文件
use mysql_async::prelude::*; // 引入 Queryable trait
use mysql_async::{Conn, Opts};
use std::env; // 引入 async-trait

use log::{error, info, warn};

// 定義 MySQL 連線管理器
pub struct MySqlConnectionManager {
    opts: Opts,
}

impl MySqlConnectionManager {
    pub fn new(database_url: String) -> Self {
        let opts = Opts::from_url(&database_url).expect("Invalid MySQL URL");
        Self { opts }
    }
}

#[async_trait]
impl ManageConnection for MySqlConnectionManager {
    type Connection = Conn;
    type Error = mysql_async::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Conn::new(self.opts.clone()).await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        // 使用簡單的查詢來驗證連線
        conn.query_drop("SELECT 1").await
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        // 無法在這裡執行非同步檢查，因此簡單地返回 false
        false
    }
}

// 初始化資料庫連線池
pub async fn init_db_pool() -> Pool<MySqlConnectionManager> {
    // 加載 .env 文件
    dotenv().ok();

    // 從環境變數或配置檔案讀取資料庫連接字串
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Db URL: {}", database_url);
    warn!("Db URL: {}", database_url);
    error!("Db URL: {}", database_url);

    // 建立 MySQL 連線管理器
    let manager = MySqlConnectionManager::new(database_url);

    // 建立 bb8 連線池
    Pool::builder().max_size(15).build(manager).await.unwrap()
}

// 將 Option<String> 轉換為 Option<NaiveDateTime>
pub fn parse_datetime(datetime_str: Option<String>) -> Option<NaiveDateTime> {
    datetime_str.and_then(|d| NaiveDateTime::parse_from_str(&d, "%Y-%m-%d %H:%M:%S").ok())
}

// 將 Option<String> 轉換為 Option<NaiveDate>
pub fn parse_date(date_str: Option<String>) -> Option<NaiveDate> {
    date_str.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok())
}