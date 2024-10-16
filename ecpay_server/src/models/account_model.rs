use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub uid: i32,
    pub user: String,
    pub password: String,
    pub authority: i32,
    pub stores: String,
    pub cid: i32,
    pub is_modify_member: i32,
    pub stations: String,
}
