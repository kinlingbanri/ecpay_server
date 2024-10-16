use crate::models::account_model::Account;
use bb8::Pool;
//use mysql_async::Conn;
use crate::db::MySqlConnectionManager;
use mysql_async::prelude::*;
use mysql_async::Error;

pub struct AccountRepository {
    pub pool: Pool<MySqlConnectionManager>,
}

impl AccountRepository {
    // 查詢所有帳號
    pub async fn find_all(&self) -> Result<Vec<Account>, Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| mysql_async::Error::Other(Box::new(e)))?;
        let result = conn
            .exec::<(i32, String, String, i32, String, i32, i32, String), _, _>(
                "SELECT uid, user, password, authority, stores, cid, is_modify_member, stations FROM account",
                (),  // 空的查詢參數
            )
            .await?;

        let accounts: Vec<Account> = result
            .into_iter()
            .map(
                |(uid, user, password, authority, stores, cid, is_modify_member, stations)| {
                    Account {
                        uid,
                        user,
                        password,
                        authority,
                        stores,
                        cid,
                        is_modify_member,
                        stations,
                    }
                },
            )
            .collect();

        Ok(accounts)
    }
    /*
    // 查詢帳號 by id
    pub async fn get_by_id(&self, uid: i32) -> Result<Account, Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| mysql_async::Error::Other(Box::new(e)))?;
        let result = conn
            .exec_first::<(i32, String, String, i32, String, i32, i32, String), _, _>(
                "SELECT uid, user, password, authority, stores, cid, is_modify_member, stations FROM account WHERE uid = ?",
                (uid,),
            )
            .await?;

        result
            .map(
                |(uid, user, password, authority, stores, cid, is_modify_member, stations)| {
                    Account {
                        uid,
                        user,
                        password,
                        authority,
                        stores,
                        cid,
                        is_modify_member,
                        stations,
                    }
                },
            )
            .ok_or_else(|| {
                mysql_async::Error::Other(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Account not found",
                )))
            })
    }
    */
}
