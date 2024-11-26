use std::sync::Arc;
use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};
use utilites::Date;

use crate::users::{Status, User};

use super::ConnectionPool;

impl FromRow<'_, SqliteRow> for User
{
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
    {
        let id: u64 =  row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let nick: Option<String> = row.try_get("nick")?;
        let updated: &str = row.try_get("updated")?;
        let updated = Date::parse(updated).unwrap();
        let status: String = row.try_get("status")?;
        let status = serde_json::from_str(&status).unwrap();
        Ok(Self::new(id, name, nick, updated, status))
    }
}

fn create_table<'a>() -> &'a str
{
    "BEGIN;
    CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    nick TEXT,
    updated TEXT NOT NULL,
    current_status TEXT NOT NULL
    );
    CREATE INDEX users_idx IF NOT EXISTS ON users (id, current_status);
    COMMIT;"
}




pub trait IUserRepository : ConnectionPool
{
    async fn create(&self)
    {
        let pool = self.get_pool();
        let result = sqlx::query(create_table()).execute(&*pool).await;
    }
    async fn add(&self, user: &User)
    {
        let pool = self.get_pool();
        let query = format!("INSERT OR IGNORE INTO users (id, username, nick, updated, current_status) VALUES ({}, $1, $2, $3, $4)", &user.id);
        let _result = sqlx::query(&query)
        .bind(&user.username)
        .bind(user.nick.as_ref())
        .bind(user.updated.format(utilites::DateFormat::Serialize))
        .bind(serde_json::to_string(&user.current_status).unwrap())
        .execute(&*pool).await;
    }
    async fn set_status_for_all(&self, status: Status)
    {
        let pool = self.get_pool();
        let query = "UPDATE users SET current_status = $1";
        let _result = sqlx::query(&query)
        .bind(serde_json::to_string(&status).unwrap())
        .execute(&*pool).await;
    }
}

pub struct UserRepository
{
    pool: Arc<SqlitePool>
}
impl UserRepository
{
    pub fn new(pool: Arc<SqlitePool>) -> Self
    {
        Self
        {
            pool
        }
    }
}

impl ConnectionPool for UserRepository
{
    fn get_pool(&self) -> Arc<SqlitePool> 
    {
        Arc::clone(&self.pool)
    }
}
impl IUserRepository for UserRepository{}

#[cfg(test)]
mod tests
{
    use std::sync::Arc;

    use crate::db::user_repository::IUserRepository;

    #[tokio::test]
    async fn test_db()
    {
        let pool = Arc::new(super::super::connection::new_connection("bot").await.unwrap());
        let ur = super::UserRepository::new(pool);
        ur.create().await;
    }
}