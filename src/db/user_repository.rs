use std::sync::Arc;
use futures::future::BoxFuture;
use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};
use utilites::Date;
use crate::users::{Status, User};
use super::ConnectionPool;

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
    CREATE INDEX IF NOT EXISTS 'users_idx' ON users (id, current_status);
    COMMIT;"
}


pub trait IUserRepository : ConnectionPool
{
    async fn create(&self) -> BoxFuture<'a, Result<(), crate::error::Error>>
    {
        let pool = self.get_pool();
        let r = sqlx::query(create_table()).execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }
    /// add or on conflict id update all user data
    async fn add(&self, user: &User) -> BoxFuture<'a, Result<(), crate::error::Error>>
    {
        let pool = self.get_pool();
        let query = format!("INSERT INTO users (id, username, nick, updated, current_status) VALUES ({}, $1, $2, $3, $4) 
        ON CONFLICT(id) DO UPDATE SET username = excluded.username, nick = excluded.nick, updated = excluded.updated, current_status = excluded.current_status WHERE id = excluded.id", &user.id);
        let r = sqlx::query(&query)
        .bind(&user.username)
        .bind(user.nick.as_ref())
        .bind(user.updated.format(utilites::DateFormat::Serialize))
        .bind(serde_json::to_string(&user.current_status).unwrap())
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }
    async fn set_status_for_all(&self, status: Status) -> BoxFuture<'a, Result<(), crate::error::Error>>
    {
        let pool = self.get_pool();
        let query = "UPDATE users SET current_status = $1";
        let r = sqlx::query(&query)
        .bind(serde_json::to_string(&status).unwrap())
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }
    async fn change_status(&self, user_id: u64, status: Status) -> BoxFuture<'a, Result<(), crate::error::Error>>
    {
        let pool = self.get_pool();
        let query = format!("UPDATE users SET current_status = $1 WHERE id = {}", user_id);
        let r = sqlx::query(&query)
        .bind(serde_json::to_string(&status).unwrap())
        .execute(&*pool).await;
        if r.is_err()
        {
            logger::error!("{}", r.as_ref().err().unwrap());
            let _ = r?;
        };
        Ok(())
    }
}

pub struct UserRepository
{
    pool: Arc<SqlitePool>
}
impl UserRepository
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
        let ur = super::UserRepository::new(pool).await;
        ur.create().await;
    }
}