use std::{error::Error, sync::Arc};
use sqlx::{encode::IsNull, error::BoxDynError, sqlite::SqliteRow, Any, Database, Decode, Encode, FromRow, Row, SqlitePool, Value};
use utilites::Date;
use crate::users::{Status, User, UsersState};
use super::ConnectionPool;

#[derive(FromRow)]
pub struct GroupDbo
{
    chat_id: i64,
    users_count: u32,
    group_name: Option<String>,
    is_active: bool
}

struct UserPlusCount
{
    user: User,
    count: u32
}

impl FromRow<'_, SqliteRow> for UserPlusCount
{
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
    {
        let id: u64 =  row.try_get("id")?;
        let username: String = row.try_get("name")?;
        let nick: Option<String> = row.try_get("nick")?;
        let updated: &str = row.try_get("updated")?;
        let updated = Date::parse(updated).unwrap();
        let status: String = row.try_get("status")?;
        let status = serde_json::from_str(&status).unwrap();
        let count: u32 = row.try_get("users_count")?;
        let user = User::new(id, username, nick, updated, status);
        Ok(UserPlusCount { user, count })
    }
}


fn select_chat_states_query(chat_id: i64) -> String
{
    format!("SELECT u.id, u.name, u.nick, u.updated, u.current_status, gr.users_count, 
    from users as u
    left JOIN chat_id_user_id as cu on cu.user_id = u.id
    left JOIN groups as gr
    where gr.chat_id = {}", chat_id)
}
fn create_groups_table_query<'a>() -> &'a str
{
    "BEGIN;
    CREATE TABLE IF NOT EXISTS groups (
    chat_id INTEGER NOT NULL PRIMARY KEY,
    users_count INTEGER DEFAULT 0,
    group_name TEXT,
    is_active INTEGER DEFAULT 0
    );
    CREATE INDEX groups_idx IF NOT EXISTS ON groups (chat_id);
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
    CREATE INDEX chat_id_user_id_idx IF NOT EXISTS ON chat_id_user_id (chat_id);
    COMMIT;"
}


pub trait IGroupRepository : ConnectionPool
{
    async fn create(&self)
    {
        let pool = self.get_pool();
        let _ = sqlx::query(create_groups_table_query()).execute(&*pool).await;
        let _ = sqlx::query(create_complex_table_query()).execute(&*pool).await;
    }

    async fn add_chat(&self, chat_id: i64)
    {
        let pool = self.get_pool();
        let sql = "INSERT OR IGNORE INTO groups (chat_id, users_count, group_name, is_active) VALUES ($1, $2, $3, $4)";
        sqlx::query(sql)
        .bind(chat_id)
        .bind(1)
        .bind(None::<String>)
        .bind(false)
        .execute(&*pool).await;
    }

    async fn add_user_to_chat(&self, chat_id: i64, user_id: u64)
    {
        let pool = self.get_pool();
        let sql = format!("INSERT OR IGNORE INTO chat_id_user_id (chat_id, user_id) VALUES ($1, {})", user_id);
        sqlx::query(&sql)
        .bind(chat_id)
        .execute(&*pool).await;
    }
    async fn get_users_state(&self, chat_id: i64) -> Result<UsersState, crate::error::Error>
    {
        let pool = self.get_pool();
        let res = sqlx::query_as::<_, UserPlusCount>(&select_chat_states_query(chat_id))
        .fetch_all(&*pool).await?;
        if res.len() > 0
        {
            let count = res[0].count;
            return Ok(UsersState::new(res.into_iter().map(|m| m.user).collect(), count))
        }
        else 
        {
            logger::info!("в чате {} не найдено ни одного связанного юзера", {chat_id});
            return Ok(UsersState::default())    
        };
    }

}


pub struct GroupRepository
{
    pool: Arc<SqlitePool>
}
impl GroupRepository
{
    pub fn new(pool: Arc<SqlitePool>) -> Self
    {
        Self
        {
            pool
        }
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
