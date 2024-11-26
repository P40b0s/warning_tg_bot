use std::sync::Arc;

use sqlx::SqlitePool;

mod connection;
mod user_repository;
mod group_repository;
mod chat_id_user_id;
pub use group_repository::{IGroupRepository, GroupRepository};
pub use user_repository::{IUserRepository, UserRepository};
pub use connection::new_connection;

pub trait ConnectionPool
{
    fn get_pool(&self) -> Arc<SqlitePool>;
}