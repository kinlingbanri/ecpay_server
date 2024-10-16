use crate::models::cluster_model::ClusterVO;
use crate::repositories::cluster_repository::ClusterRepository;
use mysql_async::Error;
use std::io;

// 定義 Result 別名，簡化回傳型別
pub type ClusterResult<T> = Result<T, Error>;

pub struct ClusterService {
    repository: ClusterRepository,
}

impl ClusterService {
    // 創建一個新的 ClusterService 實例
    pub fn new(repository: ClusterRepository) -> Self {
        ClusterService { repository }
    }

    // 根據ID獲取 Cluster 資料
    pub async fn get_cluster_by_id(&self, cid: i32) -> ClusterResult<Option<ClusterVO>> {
        if cid <= 0 {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid CID value",
            ))));
        }
        self.repository.get_one_by_id(cid).await
    }
    
    // 根據ID更新 Cluster
    pub async fn update_cluster(&self, cluster: &ClusterVO) -> ClusterResult<u64> {
        // 參數驗證邏輯，例如驗證 `cid` 是否有效
        if cluster.cid <= 0 {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid CID value",
            ))));
        }

        self.repository.update_cluster(cluster).await
    }

    // 根據ID獲取付款相關的 Cluster 資料
    pub async fn get_one_pay_by_cid(&self, cid: i32) -> ClusterResult<Option<ClusterVO>> {
        if cid <= 0 {
            return Err(Error::Other(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid CID value",
            ))));
        }

        self.repository.get_one_pay_by_cid(cid).await
    }
}
