use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfigVO {
    pub id: i32,
    pub ocpp_port: i32,

    pub ocpp_ftp_ip: Option<String>, // 改為 Option<String>
    pub ocpp_ftp_port: i32,
    pub ocpp_ftp_path: Option<String>, // 改為 Option<String>
    pub ocpp_ftp_user: Option<String>, // 改為 Option<String>
    pub ocpp_ftp_pwd: Option<String>,  // 改為 Option<String>
    pub is_enc_key: Option<u8>,
    pub backup_disk_path: String,
    pub product: String,

    pub ip: Option<String>,
    pub port: i32,
}
