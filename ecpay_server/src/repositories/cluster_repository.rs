use crate::db::MySqlConnectionManager;
use crate::models::cluster_model::ClusterVO;
use bb8::Pool;
use mysql_async::prelude::*;
use mysql_async::Error;
use mysql_async::Row;
use mysql_async::Value;

pub struct ClusterRepository {
    pub pool: Pool<MySqlConnectionManager>,
}

impl ClusterRepository {
    // 透過 cid 查詢單一 Cluster
    pub async fn get_one_by_id(&self, cid: i32) -> Result<Option<ClusterVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Option<Row> = conn
        .exec_first(
            "SELECT cid, name, is_handsel, handsel_point, conservator, vip1, vip2, vip3, general, smsId, sms_message, pay_state, line_key, line_channel_Id, line_confirm_url, line_cancel_url FROM cluster WHERE cid = ?",
            (cid,)
        )
        .await
        .map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to execute query: {}", e),
            )))
        })?;

        let cluster = result.map(|row| ClusterVO {
            cid: row.get(0).unwrap_or_default(),
            name: row.get(1).unwrap_or_default(),
            is_handsel: row.get(2).unwrap_or_default(),
            handsel_point: row.get(3).unwrap_or_default(),
            conservator: row.get(4).unwrap_or_default(),
            vip1: row.get(5).unwrap_or_default(),
            vip2: row.get(6).unwrap_or_default(),
            vip3: row.get(7).unwrap_or_default(),
            general: row.get(8).unwrap_or_default(),
            sms_id: row.get(9).unwrap_or_default(),
            sms_message: row.get(10).unwrap_or_default(),
            pay_state: row.get(11).unwrap_or_default(),
            line_key: row.get(12).unwrap_or_default(),
            line_channel_id: row.get(13).unwrap_or_default(),
            line_confirm_url: row.get(14).unwrap_or_default(),
            line_cancel_url: row.get(15).unwrap_or_default(),
        });

        // 提前釋放連接資源
        drop(conn);

        Ok(cluster)
    }

    // 透過 cid 查詢付款相關資訊
    pub async fn get_one_pay_by_cid(&self, cid: i32) -> Result<Option<ClusterVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result = conn
        .exec_first::<(i32, u8, String, String, String, String), _, _>(
            "SELECT cid, pay_state, line_key, line_channel_id, line_confirm_url, line_cancel_url FROM cluster WHERE cid = ?",
            (cid,)
        )
        .await
        .map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to execute query: {}", e),
            )))
        })?;

        let cluster = result.map(
            |(cid, pay_state, line_key, line_channel_id, line_confirm_url, line_cancel_url)| {
                ClusterVO {
                    cid,
                    name: String::new(), // 這裡暫時設為空字串，因為不從查詢中取得
                    sms_id: String::new(), // 這裡暫時設為空字串
                    sms_message: String::new(), // 這裡暫時設為空字串
                    is_handsel: 0,       // 這裡暫時設為0
                    handsel_point: 0,    // 這裡暫時設為0
                    pay_state,
                    line_key,
                    line_channel_id,
                    line_confirm_url,
                    line_cancel_url,
                    conservator: 0, // 這裡暫時設為0
                    vip1: 0,        // 這裡暫時設為0
                    vip2: 0,        // 這裡暫時設為0
                    vip3: 0,        // 這裡暫時設為0
                    general: 0,     // 這裡暫時設為0
                }
            },
        );

        drop(conn); // 提前釋放連接

        Ok(cluster)
    }

    // 更新 Cluster
    pub async fn update_cluster(&self, cluster: &ClusterVO) -> Result<u64, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        // 構造 SQL 更新語句的參數，避免不必要的克隆
        let params: Vec<Value> = vec![
            Value::from(&cluster.name), // 使用引用來避免克隆
            Value::from(cluster.is_handsel),
            Value::from(cluster.handsel_point),
            Value::from(cluster.conservator),
            Value::from(cluster.vip1),
            Value::from(cluster.vip2),
            Value::from(cluster.vip3),
            Value::from(cluster.general),
            Value::from(&cluster.sms_id),      // 使用引用來避免克隆
            Value::from(&cluster.sms_message), // 使用引用來避免克隆
            Value::from(cluster.pay_state),
            Value::from(&cluster.line_key), // 使用引用來避免克隆
            Value::from(&cluster.line_channel_id), // 使用引用來避免克隆
            Value::from(&cluster.line_confirm_url), // 使用引用來避免克隆
            Value::from(&cluster.line_cancel_url), // 使用引用來避免克隆
            Value::from(cluster.cid),
        ];

        conn.exec_drop(
        "UPDATE cluster SET name = ?, is_handsel = ?, handsel_point = ?, conservator = ?, vip1 = ?, vip2 = ?, vip3 = ?, general = ?, smsId = ?, sms_message = ?, pay_state = ?, line_key = ?, line_channel_Id = ?, line_confirm_url = ?, line_cancel_url = ? WHERE cid = ?",
            params
        ).await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to execute update: {}", e),
            )))
        })?;

        let affected_rows = conn.affected_rows();
        drop(conn);
        Ok(affected_rows)
    }
}
