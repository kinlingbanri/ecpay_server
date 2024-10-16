use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MemVO {
    pub mid: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub point: f32,
    pub black: Option<i32>,
    pub authority: Option<i32>,
    pub verification: Option<i32>,
    pub verificationcode: Option<String>,
    pub verificationdate: Option<String>, // 保持為 String
    pub phone: Option<String>,
    pub now_money: Option<i32>,
    pub add_money: Option<i32>,
    pub add_status: Option<i32>,
    pub add_life_status: Option<i32>,
    pub gender: Option<i32>,
    pub birthday: Option<String>, // 保持為 String
    pub transport_time: Option<i32>,
    pub transportation: Option<String>,
    pub cid: Option<i32>,
    pub cluster_name: Option<String>,
    pub ln_access_token: Option<String>,
    pub transaction_id: i32,
    pub card: Option<String>,
    pub identity: Option<i32>,
    pub discount: Option<i32>,
    pub judge: Option<i32>,
    pub note: Option<String>,
    pub auth_pileid: Option<String>,
}
