use crate::models::account_model::Account;
use crate::repositories::account_repository::AccountRepository;

pub struct AccountService {
    pub repository: AccountRepository,
}

impl AccountService {
    /*
    // 根據 ID 查詢帳號
    pub async fn get_account(&self, uid: i32) -> Result<Account, String> {
        self.repository
            .get_by_id(uid)
            .await
            .map_err(|e| format!("Error retrieving account: {}", e))
    }
    */
    // 查詢所有帳號
    pub async fn get_all_accounts(&self) -> Result<Vec<Account>, String> {
        self.repository
            .find_all()
            .await
            .map_err(|e| format!("Error retrieving accounts: {}", e))
    }
}
