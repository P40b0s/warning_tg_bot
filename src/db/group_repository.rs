use std::{error::Error, sync::Arc};
use sqlx::{encode::IsNull, error::BoxDynError, sqlite::SqliteRow, Any, Database, Decode, Encode, FromRow, Row, SqlitePool, Value};
use utilites::Date;
use crate::users::{Group, GroupSettings, Status, User};
use super::ConnectionPool;

// #[derive(FromRow)]
// pub struct GroupDbo
// {
//     chat_id: i64,
//     users_count: u32,
//     group_name: Option<String>,
//     is_active: bool
// }


impl FromRow<'_, SqliteRow> for User
{
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
    {
        let id: u64 =  row.try_get("id")?;
        let username: String = row.try_get("username")?;
        let nick: Option<String> = row.try_get("nick")?;
        let updated: &str = row.try_get("updated")?;
        let updated = Date::parse(updated).unwrap();
        let status: String = row.try_get("current_status")?;
        let status = serde_json::from_str(&status).unwrap();
        let user = User::new(id, username, nick, updated, status);
        Ok(user)
    }
}
impl FromRow<'_, SqliteRow> for GroupSettings
{
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
    {
        let chat_id: i64 =  row.try_get("chat_id")?;
        let users_count: u32 =  row.try_get("users_count")?;
        let is_active: bool =  row.try_get("is_active")?;
        let group_name: Option<String> = row.try_get("group_name")?;
        let deadline: &str = row.try_get("deadline_time")?;
        let deadline_time = Date::parse(deadline).unwrap();
        let additional_dates: serde_json::Value = row.try_get("additional_dates")?;
        let additional_dates: Vec<Date> = serde_json::from_value(additional_dates).unwrap();
        Ok(GroupSettings 
        {
            chat_id,
            users_count,
            is_active,
            group_name,
            deadline_time,
            additional_dates
        })
    }
}

fn select_users_query(chat_id: i64) -> String
{
    format!("SELECT u.id,  u.username, u.nick, u.updated, u.current_status
            FROM chat_id_user_id AS cu
            LEFT JOIN users AS u ON cu.user_id = u.id
            WHERE  cu.chat_id = {} 
            ORDER BY u.updated DESC", chat_id)
}
fn select_group_settings_query(chat_id: i64) -> String
{
    format!("SELECT chat_id, users_count, deadline_time, is_active, group_name, additional_dates
            FROM groups
            WHERE  chat_id = {}", chat_id)
}
fn get_real_users_count_query(chat_id: i64) -> String
{
    format!("SELECT COUNT(u.username)
            FROM chat_id_user_id AS cu
            LEFT JOIN users AS u ON cu.user_id = u.id
            LEFT JOIN groups AS gr ON gr.chat_id = cu.chat_id
            WHERE  cu.chat_id = {}", chat_id)
}
fn create_groups_table_query<'a>() -> &'a str
{
    "BEGIN;
    CREATE TABLE IF NOT EXISTS groups (
    chat_id INTEGER NOT NULL PRIMARY KEY,
    users_count INTEGER DEFAULT 0,
    group_name TEXT,
    is_active INTEGER DEFAULT 0,
    deadline_time TEXT DEFAULT '12:00:00',
    additional_dates TEXT DEFAULT('[]')
    );
    CREATE INDEX IF NOT EXISTS 'groups_idx' ON groups (chat_id);
    COMMIT;"
}

fn create_complex_table_query<'a>() -> &'a str
{
    "BEGIN;
    CREATE TABLE IF NOT EXISTS chat_id_user_id (
    chat_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    PRIMARY KEY(chat_id, user_id)
    );
    CREATE INDEX IF NOT EXISTS 'chat_id_user_id_idx' ON chat_id_user_id (chat_id);
    COMMIT;"
}


pub trait IGroupRepository : ConnectionPool
{
    async fn create(&self) -> Result<(), crate::error::Error>
    {
        let pool = self.get_pool();
        let r1 = sqlx::query(create_groups_table_query()).execute(&*pool).await;
        if r1.is_err()
        {
            logger::error!("{}", r1.as_ref().err().unwrap());
            let _ = r1?;
        };
        let r2 = sqlx::query(create_complex_table_query()).execute(&*pool).await;
        if r2.is_err()
        {
            logger::error!("{}", r2.as_ref().err().unwrap());
            let _ = r2?;
        };
        Ok(())
    }

    async fn add_chat(&self, chat_id: i64) -> Result<(), crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = "INSERT OR IGNORE INTO groups (chat_id, users_count, group_name, is_active) VALUES ($1, $2, $3, $4)";
        let r = sqlx::query(sql)
        .bind(chat_id)
        .bind(1)
        .bind(None::<String>)
        .bind(false)
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }

    async fn add_user_to_chat(&self, chat_id: i64, user_id: u64) -> Result<(), crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = format!("INSERT OR IGNORE INTO chat_id_user_id (chat_id, user_id) VALUES ($1, {})", user_id);
        let r = sqlx::query(&sql)
        .bind(chat_id)
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        let users_count = self.get_users_count(chat_id).await?;
        let real_users_count = self.get_real_users_count(chat_id).await?;
        if real_users_count > users_count
        {
            self.set_users_count(chat_id, real_users_count).await?;
        }
        Ok(())
    }
    async fn remove_user_from_chat(&self, chat_id: i64, user_id: u64) -> Result<(), crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = format!("DELETE FROM chat_id_user_id WHERE chat_id = $1 AND user_id = {}", user_id);
        let r = sqlx::query(&sql)
        .bind(chat_id)
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }
    /// Get all users and total count for current chat id + this group settings
    async fn get_group(&self, chat_id: i64) -> Result<Group, crate::error::Error>
    {
        let settings = self.get_group_settings(chat_id).await?;
        let users = self.get_users(chat_id).await?;
        return Ok(Group::new(chat_id, users).add_settings(settings));    
    }
    async fn get_group_settings(&self, chat_id: i64) -> Result<GroupSettings, crate::error::Error>
    {
        let pool = self.get_pool();
        let settings = sqlx::query_as::<_, GroupSettings>(&select_group_settings_query(chat_id))
        .fetch_one(&*pool).await;
        if settings.is_err()
        {
            logger::error!("{}", settings.as_ref().err().unwrap());
            return Err(crate::Error::SqlxError(settings.err().unwrap()));
        }
        return Ok(settings.unwrap());    
    }

    async fn get_users(&self, chat_id: i64) -> Result<Vec<User>, crate::error::Error>
    {
        let pool = self.get_pool();
        let r = sqlx::query_as::<_, User>(&select_users_query(chat_id))
        .fetch_all(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            return Err(crate::Error::SqlxError(r.err().unwrap()));
        }
        return Ok(r.unwrap());    
    }

    /// Authorization check, if the chat is not found, it will be added to the table
    async fn chat_is_authorized(&self, chat_id: i64) -> Result<bool, crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = "SELECT is_active FROM groups WHERE chat_id = ?";
        let r: Result<Option<(bool,)>, sqlx::Error> = sqlx::query_as(&sql)
        .bind(chat_id)
        .fetch_optional(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            return Err(crate::Error::SqlxError(r.err().unwrap()));
        }
        match r.as_ref().unwrap()
        {
            Some(r) => Ok(r.0),
            None => 
            {
                self.add_chat(chat_id).await?;
                Ok(false)
            }
        }
    }
    async fn get_users_count(&self, chat_id: i64) -> Result<u32, crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = "SELECT users_count FROM groups WHERE chat_id = ?";
        let r: Result<Option<(u32,)>, sqlx::Error> = sqlx::query_as(&sql)
        .bind(chat_id)
        .fetch_optional(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            return Err(crate::Error::SqlxError(r.err().unwrap()));
        }
        match r.as_ref().unwrap()
        {
            Some(r) => Ok(r.0),
            None => 
            {
                self.add_chat(chat_id).await?;
                Ok(0)
            }
        }
    }
    async fn get_real_users_count(&self, chat_id: i64) -> Result<u32, crate::error::Error>
    {
        let pool = self.get_pool();
        let r: Result<(u32,), sqlx::Error> = sqlx::query_as(&get_real_users_count_query(chat_id))
        .bind(chat_id)
        .fetch_one(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            return Err(crate::Error::SqlxError(r.err().unwrap()));
        }
        Ok(r.unwrap().0)
    }
    async fn set_users_count(&self, chat_id: i64, count: u32) -> Result<(), crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = "UPDATE groups SET users_count = $1 WHERE chat_id = $2";
        let r = sqlx::query(&sql)
        .bind(count)
        .bind(chat_id)
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }
    async fn set_deadline_time(&self, chat_id: i64, time: Date) -> Result<(), crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = "UPDATE groups SET deadline_time = $1 WHERE chat_id = $2";
        let r = sqlx::query(&sql)
        .bind(time.format(utilites::DateFormat::Time))
        .bind(chat_id)
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }
    async fn get_additional_dates(&self, chat_id: i64) -> Result<Vec<Date>, crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = "SELECT additional_dates FROM groups WHERE chat_id = ?";
        let r: Result<(serde_json::Value,), sqlx::Error> = sqlx::query_as(&sql)
        .bind(chat_id)
        .fetch_one(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            return Err(crate::Error::SqlxError(r.err().unwrap()));
        }
        Ok(serde_json::from_value(r.unwrap().0).unwrap())
    }
    async fn set_additional_dates(&self, chat_id: i64, dates: Vec<Date>) -> Result<(), crate::error::Error>
    {
        let pool = self.get_pool();
        let sql = "UPDATE groups SET additional_dates = $1 WHERE chat_id = $2";
        let r = sqlx::query(&sql)
        .bind(serde_json::to_string(&dates))
        .bind(chat_id)
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }
    const KEY: &str = "713e4412-1962-47d6-9ae3-9d658b1a06c7";
    async fn register_group(&self, chat_id: i64, id: &str) -> Result<(), crate::error::Error>
    {
        let pool = self.get_pool();
        if id != Self::KEY
        {
            return Err(crate::error::Error::WrongRegisterKeyError(id.to_owned()));
        }
        let sql = "UPDATE groups SET is_active = 1 WHERE chat_id = ?";
        let r = sqlx::query(&sql)
        .bind(chat_id)
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }

}


pub struct GroupRepository
{
    pool: Arc<SqlitePool>
}
impl GroupRepository
{
    pub async fn new(pool: Arc<SqlitePool>) -> Self
    {
        
        let s = Self
        {
            pool
        };
        s.create().await;
        s
    }
}

impl ConnectionPool for GroupRepository
{
    fn get_pool(&self) -> Arc<SqlitePool> 
    {
        Arc::clone(&self.pool)
    }
}
impl IGroupRepository for GroupRepository{}

#[cfg(test)]
mod tests
{
    use std::sync::Arc;
}
