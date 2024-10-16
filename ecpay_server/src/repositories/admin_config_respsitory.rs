use crate::db::MySqlConnectionManager;
use crate::models::admin_config_model::AdminConfigVO;
use bb8::Pool;
use mysql_async::prelude::*;
use mysql_async::Error;

pub struct AdminConfigRepository {
    pub pool: Pool<MySqlConnectionManager>,
}

impl AdminConfigRepository {
    // 根据 id 查询单一 AdminConfig
    pub async fn get_one_by_id(&self, id: i32) -> Result<Option<AdminConfigVO>, Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| mysql_async::Error::Other(Box::new(e)))?;

        let result = conn
        .exec_first::<(i32, Option<String>, i32, Option<String>, Option<String>, Option<String>, Option<String>, i32, String, String, i32), _, _>(
            "SELECT id, ocpp_ftp_ip, ocpp_ftp_port, ocpp_ftp_path, ocpp_ftp_user, ocpp_ftp_pwd, ip, port, backup_disk_path, product, ocpp_port FROM adminconfig WHERE id = ?",
            (id,),
        )
        .await?;

        let admin_config = result.map(
            |(
                id,
                ocpp_ftp_ip,
                ocpp_ftp_port,
                ocpp_ftp_path,
                ocpp_ftp_user,
                ocpp_ftp_pwd,
                ip,
                port,
                backup_disk_path,
                product,
                ocpp_port,
            )| {
                AdminConfigVO {
                    id,
                    ocpp_ftp_ip: Some(ocpp_ftp_ip.unwrap_or_default()), // 包裝成 Some()
                    ocpp_ftp_port,
                    ocpp_ftp_path: Some(ocpp_ftp_path.unwrap_or_default()), // 包裝成 Some()
                    ocpp_ftp_user: Some(ocpp_ftp_user.unwrap_or_default()), // 包裝成 Some()
                    ocpp_ftp_pwd: Some(ocpp_ftp_pwd.unwrap_or_default()),   // 包裝成 Some()
                    ip: Some(ip.unwrap_or_default()),                       // 包裝成 Some()
                    port,
                    backup_disk_path, // 假設這些字段不為 None，直接使用
                    product,
                    ocpp_port,
                    is_enc_key: Some(0), // 如果是 None，默認為 0
                }
            },
        );

        Ok(admin_config)
    }

    // 获取 isEncKey 的值
    pub async fn get_is_enc_key(&self) -> Result<Option<u8>, Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| mysql_async::Error::Other(Box::new(e)))?;

        let result = conn
            .exec_first::<u8, _, _>("SELECT isEncKey FROM adminconfig WHERE id = 1", ())
            .await?;

        Ok(result)
    }

    // 更新 isEncKey 的值
    pub async fn update_is_enc_key(&self, is_enc_key: u8) -> Result<u64, Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| mysql_async::Error::Other(Box::new(e)))?;

        // 使用 exec_drop 不会返回结果，但会成功执行 SQL 更新
        conn.exec_drop(
            "UPDATE adminconfig SET isEncKey = ? WHERE id = 1",
            (is_enc_key,), // 这里是传入参数的正确元组形式
        )
        .await?;

        // 使用 affected_rows() 来获取受影响的行数
        let affected_rows = conn.affected_rows();

        Ok(affected_rows)
    }
}
