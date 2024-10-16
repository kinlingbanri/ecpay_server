use crate::db::MySqlConnectionManager;
use crate::models::mem_model::MemVO;
use bb8::Pool;
use mysql_async::prelude::*;
use mysql_async::Error;

pub struct MemRepository {
    pub pool: Pool<MySqlConnectionManager>,
}

impl MemRepository {
    // 查詢所有 Mem 資料，根據 username 排序
    // 查詢所有 Mem 資料，根據 username 排序
    pub async fn get_all(&self) -> Result<Vec<MemVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Vec<mysql_async::Row> = conn
            .query(
                "SELECT mid, username, email, password, point, black, authority, verification, \
                 verificationcode, verificationdate, phone, add_money, add_status, add_life_status, \
                 now_money, gender, birthday, transportTime, transportation, cid FROM mem ORDER BY username"
            )
            .await?;

        let mems = result
            .into_iter()
            .map(|row| MemVO {
                mid: row.get(0).unwrap_or_default(),
                username: row.get(1).unwrap_or_default(),
                email: row.get(2).unwrap_or_default(),
                password: row.get(3).unwrap_or_default(),
                point: row.get(4).unwrap_or_default(),
                black: row.get(5),
                authority: row.get(6),
                verification: row.get(7),
                verificationcode: row.get(8),
                verificationdate: row.get(9), // 保持為 Option<String>
                phone: row.get(10),
                add_money: row.get(11),
                add_status: row.get(12),
                add_life_status: row.get(13),
                now_money: row.get(14),
                gender: row.get(15),
                birthday: row.get(16), // 保持為 Option<String>
                transport_time: row.get(17),
                transportation: row.get(18),
                cid: row.get(19),
                ..Default::default() // 將未用到的欄位預設為空
            })
            .collect::<Vec<MemVO>>();

        Ok(mems)
    }

    // 查詢所有 Mem 資料，根據 phone 排序
    pub async fn get_all_by_phone(&self) -> Result<Vec<MemVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Vec<mysql_async::Row> = conn
            .query("SELECT mid, username, email, password, point, black, authority, verification, verificationcode, verificationdate, phone, add_money, add_status, add_life_status, now_money, gender, birthday, transportTime, transportation, cid FROM mem ORDER BY phone")
            .await?;

        let mems = result
            .into_iter()
            .map(|row| MemVO {
                mid: row.get(0).unwrap_or_default(),
                username: row.get(1).unwrap_or_default(),
                email: row.get(2).unwrap_or_default(),
                password: row.get(3).unwrap_or_default(),
                point: row.get(4).unwrap_or_default(),
                black: row.get(5),
                authority: row.get(6),
                verification: row.get(7),
                verificationcode: row.get(8),
                verificationdate: row.get(9),
                phone: row.get(10),
                add_money: row.get(11),
                add_status: row.get(12),
                add_life_status: row.get(13),
                now_money: row.get(14),
                gender: row.get(15),
                birthday: row.get(16),
                transport_time: row.get(17),
                transportation: row.get(18),
                cid: row.get(19),
                ..Default::default()
            })
            .collect();

        Ok(mems)
    }

    // 根據 CID 查詢所有 Mem 資料
    pub async fn get_all_by_cid(&self, cid: i32) -> Result<Vec<MemVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Vec<mysql_async::Row> = conn
            .exec("SELECT mid, username, email, password, point, black, authority, verification, phone, gender, birthday, transportTime, transportation, cid FROM mem WHERE cid = ?", (cid,))
            .await?;

        let mems = result
            .into_iter()
            .map(|row| MemVO {
                mid: row.get(0).unwrap_or_default(),
                username: row.get(1).unwrap_or_default(),
                email: row.get(2).unwrap_or_default(),
                password: row.get(3).unwrap_or_default(),
                point: row.get(4).unwrap_or_default(),
                black: row.get(5),
                authority: row.get(6),
                verification: row.get(7),
                phone: row.get(8),
                gender: row.get(9),
                birthday: row.get(10),
                transport_time: row.get(11),
                transportation: row.get(12),
                cid: row.get(13),
                ..Default::default()
            })
            .collect();

        Ok(mems)
    }

    // 根據 username 查詢單一 Mem
    pub async fn get_one_by_username(&self, username: &str) -> Result<Option<MemVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Option<mysql_async::Row> = conn
            .exec_first("SELECT mid, username, email, password, point, black, authority, verification, verificationcode, verificationdate, phone, now_money, add_money, add_status, add_life_status, gender, birthday, transportTime, transportation, cid FROM mem WHERE username = ?", (username,))
            .await?;

        let mem = result.map(|row| MemVO {
            mid: row.get(0).unwrap_or_default(),
            username: row.get(1).unwrap_or_default(),
            email: row.get(2).unwrap_or_default(),
            password: row.get(3).unwrap_or_default(),
            point: row.get(4).unwrap_or_default(),
            black: row.get(5),
            authority: row.get(6),
            verification: row.get(7),
            verificationcode: row.get(8),
            verificationdate: row.get(9),
            phone: row.get(10),
            now_money: row.get(11),
            add_money: row.get(12),
            add_status: row.get(13),
            add_life_status: row.get(14),
            gender: row.get(15),
            birthday: row.get(16),
            transport_time: row.get(17),
            transportation: row.get(18),
            cid: row.get(19),
            ..Default::default()
        });

        Ok(mem)
    }

    // 根據 email 查詢單一 Mem
    pub async fn get_one_by_email(&self, email: &str) -> Result<Option<MemVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Option<mysql_async::Row> = conn
            .exec_first("SELECT mid, username, email, password, point, black, authority, verification, verificationcode, verificationdate, phone, add_money, add_status, now_money, add_life_status, gender, birthday, transportTime, transportation, cid FROM mem WHERE email = ?", (email,))
            .await?;

        let mem = result.map(|row| MemVO {
            mid: row.get(0).unwrap_or_default(),
            username: row.get(1).unwrap_or_default(),
            email: row.get(2).unwrap_or_default(),
            password: row.get(3).unwrap_or_default(),
            point: row.get(4).unwrap_or_default(),
            black: row.get(5),
            authority: row.get(6),
            verification: row.get(7),
            verificationcode: row.get(8),
            verificationdate: row.get(9),
            phone: row.get(10),
            add_money: row.get(11),
            add_status: row.get(12),
            now_money: row.get(13),
            add_life_status: row.get(14),
            gender: row.get(15),
            birthday: row.get(16),
            transport_time: row.get(17),
            transportation: row.get(18),
            cid: row.get(19),
            ..Default::default()
        });

        Ok(mem)
    }

    // 根據 phone 查詢單一 Mem
    pub async fn get_one_by_phone(&self, phone: &str) -> Result<Option<MemVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Option<mysql_async::Row> = conn
            .exec_first("SELECT mid, username, email, password, point, black, authority, verification, verificationcode, verificationdate, phone, add_money, add_status, now_money, add_life_status, gender, birthday, transportTime, transportation, cid FROM mem WHERE phone = ?", (phone,))
            .await?;

        let mem = result.map(|row| MemVO {
            mid: row.get(0).unwrap_or_default(),
            username: row.get(1).unwrap_or_default(),
            email: row.get(2).unwrap_or_default(),
            password: row.get(3).unwrap_or_default(),
            point: row.get(4).unwrap_or_default(),
            black: row.get(5),
            authority: row.get(6),
            verification: row.get(7),
            verificationcode: row.get(8),
            verificationdate: row.get(9),
            phone: row.get(10),
            add_money: row.get(11),
            add_status: row.get(12),
            add_life_status: row.get(13),
            now_money: row.get(14),
            gender: row.get(15),
            birthday: row.get(16),
            transport_time: row.get(17),
            transportation: row.get(18),
            cid: row.get(19),
            ..Default::default()
        });

        Ok(mem)
    }

    // 根據 MID 查詢單一 Mem
    pub async fn get_one_by_mid(&self, mid: i32) -> Result<Option<MemVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Option<mysql_async::Row> = conn
            .exec_first("SELECT mid, username, email, password, point, black, authority, verification, verificationcode, verificationdate, phone, add_money, add_status, now_money, add_life_status, gender, birthday, transportTime, transportation, cid FROM mem WHERE mid = ?", (mid,))
            .await?;

        let mem = result.map(|row| MemVO {
            mid: row.get(0).unwrap_or_default(),
            username: row.get(1).unwrap_or_default(),
            email: row.get(2).unwrap_or_default(),
            password: row.get(3).unwrap_or_default(),
            point: row.get(4).unwrap_or_default(),
            black: row.get(5),
            authority: row.get(6),
            verification: row.get(7),
            verificationcode: row.get(8),
            verificationdate: row.get(9),
            phone: row.get(10),
            add_money: row.get(11),
            add_status: row.get(12),
            add_life_status: row.get(13),
            now_money: row.get(14),
            gender: row.get(15),
            birthday: row.get(16),
            transport_time: row.get(17),
            transportation: row.get(18),
            cid: row.get(19),
            ..Default::default()
        });

        Ok(mem)
    }

    // 根據 MID 查詢 LINE access token
    pub async fn get_line_access_token(&self, mid: i32) -> Result<Option<String>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Option<String> = conn
            .exec_first("SELECT ln_access_token FROM mem WHERE mid = ?", (mid,))
            .await?;

        Ok(result)
    }

    // 根據卡號查詢單一 Mem
    pub async fn get_one_by_card(&self, card: &str) -> Result<Option<MemVO>, Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            mysql_async::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get connection from pool: {}", e),
            )))
        })?;

        let result: Option<mysql_async::Row> = conn
            .exec_first("SELECT mid, transactionId, phone, point, cid, black, card, identity, discount, judge, auth_pileid, username FROM mem WHERE card = ?", (card,))
            .await?;

        let mem = result.map(|row| MemVO {
            mid: row.get(0).unwrap_or_default(),
            transaction_id: row.get(1).unwrap_or_default(),
            phone: row.get(2).unwrap_or_default(),
            point: row.get(3).unwrap_or_default(),
            cid: row.get(4).unwrap_or_default(),
            black: row.get(5),
            card: row.get(6).unwrap_or_default(),
            identity: row.get(7),
            discount: row.get(8),
            judge: row.get(9),
            auth_pileid: row.get(10).unwrap_or_default(),
            username: row.get(11).unwrap_or_default(),
            ..Default::default() // 將未用到的欄位預設為空
        });

        Ok(mem)
    }
}
