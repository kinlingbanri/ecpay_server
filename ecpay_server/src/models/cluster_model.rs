use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterVO {
    pub cid: i32,
    pub name: String,
    pub sms_id: String,
    pub sms_message: String,
    pub is_handsel: i32,
    pub handsel_point: i32,
    pub pay_state: u8,
    pub line_key: String,
    pub line_channel_id: String,
    pub line_confirm_url: String,
    pub line_cancel_url: String,
    pub conservator: i32,
    pub vip1: i32,
    pub vip2: i32,
    pub vip3: i32,
    pub general: i32,
}
