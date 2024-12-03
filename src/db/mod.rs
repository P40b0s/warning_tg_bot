use std::sync::Arc;

use sqlx::SqlitePool;

mod connection;
mod user_repository;
mod group_repository;
pub use group_repository::{IGroupRepository, GroupRepository};
pub use user_repository::{IUserRepository, UserRepository};
pub use connection::new_connection;

use crate::users::{User, Group};

pub trait ConnectionPool
{
    fn get_pool(&self) -> Arc<SqlitePool>;
}

pub struct Repository
{
    
    pub users_repository : Box<dyn IUserRepository>,
    pub groups_repository: Box<dyn IGroupRepository>
}


impl Repository
{
    pub async fn new() -> Self
    {
        let pool = Arc::new(new_connection("bot").await.unwrap());
        Self
        {
            groups_repository,
            users_repository
        }
    }
    pub async fn add_user(&self, user: &User, chat_id: i64) -> Result<Group, crate::error::Error>
    {
        self.users_repository.add(&user).await?;
        self.groups_repository.add_user_to_chat(chat_id, user.id).await?;
        self.groups_repository.get_group(chat_id).await
    }
}