use crate::models::mem_model::MemVO;
use crate::repositories::mem_repository::MemRepository;
use mysql_async::Error;
use std::io;

// 定義 Result 別名，簡化回傳型別
pub type MemResult<T> = Result<T, Error>;

pub struct MemService {
    repository: MemRepository,
}

impl MemService {
    // 創建一個新的 MemService 實例
    pub fn new(repository: MemRepository) -> Self {
        MemService { repository }
    }

    // 查詢所有 Mem 資料，根據 username 排序
    pub async fn get_all(&self) -> MemResult<Vec<MemVO>> {
        self.repository.get_all().await
    }

    // 查詢所有 Mem 資料，根據 phone 排序
    pub async fn get_all_by_phone(&self) -> MemResult<Vec<MemVO>> {
        self.repository.get_all_by_phone().await
    }

    // 根據 CID 查詢所有 Mem 資料
    pub async fn get_all_by_cid(&self, cid: i32) -> MemResult<Vec<MemVO>> {
        if cid <= 0 {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid CID value",
            ))));
        }
        self.repository.get_all_by_cid(cid).await
    }

    // 根據 username 查詢單一 Mem
    pub async fn get_mem_by_username(&self, username: &str) -> MemResult<Option<MemVO>> {
        if username.is_empty() {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Username cannot be empty",
            ))));
        }
        self.repository.get_one_by_username(username).await
    }

    // 根據 email 查詢單一 Mem
    pub async fn get_mem_by_email(&self, email: &str) -> MemResult<Option<MemVO>> {
        if email.is_empty() {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Email cannot be empty",
            ))));
        }
        self.repository.get_one_by_email(email).await
    }

    // 根據 phone 查詢單一 Mem
    pub async fn get_mem_by_phone(&self, phone: &str) -> MemResult<Option<MemVO>> {
        if phone.is_empty() {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Phone cannot be empty",
            ))));
        }
        self.repository.get_one_by_phone(phone).await
    }

    // 根據 MID 查詢單一 Mem
    pub async fn get_mem_by_mid(&self, mid: i32) -> MemResult<Option<MemVO>> {
        if mid <= 0 {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid MID value",
            ))));
        }
        self.repository.get_one_by_mid(mid).await
    }

    // 根據 MID 查詢 LINE access token
    pub async fn get_line_access_token(&self, mid: i32) -> MemResult<Option<String>> {
        if mid <= 0 {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid MID value",
            ))));
        }
        self.repository.get_line_access_token(mid).await
    }

    // 根據卡號查詢單一 Mem
    pub async fn get_mem_by_card(&self, card: &str) -> MemResult<Option<MemVO>> {
        if card.is_empty() {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Card cannot be empty",
            ))));
        }
        self.repository.get_one_by_card(card).await
    }
}
