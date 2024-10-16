use crate::models::admin_config_model::AdminConfigVO;
use crate::repositories::admin_config_respsitory::AdminConfigRepository;

use mysql_async::Error;

pub struct AdminConfigService {
    repository: AdminConfigRepository,
}

impl AdminConfigService {
    pub fn new(repository: AdminConfigRepository) -> Self {
        AdminConfigService { repository }
    }

    // 根據ID獲取AdminConfig
    pub async fn get_admin_config_by_id(&self, id: i32) -> Result<Option<AdminConfigVO>, Error> {
        self.repository.get_one_by_id(id).await
    }

    #[allow(unused)]
    // 獲取isEncKey值
    pub async fn get_is_enc_key(&self) -> Result<Option<u8>, Error> {
        self.repository.get_is_enc_key().await
    }

    #[allow(unused)]
    // 更新isEncKey
    pub async fn update_is_enc_key(&self, is_enc_key: u8) -> Result<u64, Error> {
        self.repository.update_is_enc_key(is_enc_key).await
    }
}
